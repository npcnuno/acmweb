use crate::{
    // admin::Admin,
    dbconn::{self, redis_del, redis_get, redis_set, DB},
};
use chrono::{Date, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{any::Any, pin::Pin};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{transport::Body, Request, Response, Status};
use tracing_subscriber::fmt::format;
use userposts_module::userposts_administration_server::UserpostsAdministration;

use self::userposts_module::{
    Ack, Author as Au, GetPost, GetPosts, ImageId, Lang, Post as P, PostHeader as PH, PostId,
    ServePost, ServePosts,
};

pub mod userposts_module {
    use tonic::include_proto;
    include_proto!("userposts");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("userposts_descriptor");
}
use surrealdb::{sql::Thing, Error};
const IMAGES_PATH: &str = "images/posts/";
use tokio::sync::mpsc;

use surrealdb::RecordId;

#[derive(Debug, Clone, Deserialize)]
pub struct ID {
    pub id: Thing,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    id: Thing,
    pub lang: Thing,
    title: String,
    description: String,
    markdown: String,
    author: Thing,
    date: String,
    image_id: Thing,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostHeader {
    id: Thing,
    pub lang: Language,
    title: String,
    description: String,
    pub author: Author,
    date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    id: String,
    code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguagesField {
    id: Thing,
    code: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageID {
    id: String,
}
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UserpostsManager {}

#[tonic::async_trait]
impl UserpostsAdministration for UserpostsManager {
    async fn get_posts(&self, request: Request<GetPosts>) -> Result<Response<ServePosts>, Status> {
        println!("[DEBUG] get_posts request received: {:?}", request);

        // Step 1: Fetch the list of supported languages.
        let mut q = DB.query("SELECT * FROM languages").await.map_err(|err| {
            eprintln!("[ERROR] Error fetching languages: {:?}", err);
            return Status::internal("Database error");
        })?;

        println!("[DEBUG] Languages query executed successfully.");

        let result = q.take(0);
        println!("[DEBUG] Languages query result: {:?}", result);

        let l: Vec<Option<LanguagesField>> = result.ok().unwrap_or_default();
        if l.is_empty() {
            eprintln!("[ERROR] No languages found");
            return Err(Status::aborted("No languages found"));
        }

        // Step 2: Extract and trim the requested language from the gRPC request.
        let request_lang: String = request.into_inner().lang;
        println!("[DEBUG] Requested language: {}", request_lang);

        if request_lang.len() != 2 {
            return Err(Status::invalid_argument("Invalid language code"));
        }

        // Step 3: Verify the requested language exists in the list.
        let lang_exists = l.iter().any(|l| {
            l.as_ref()
                .map(|language| language.code == request_lang)
                .unwrap_or(false)
        });
        if !lang_exists {
            return Err(Status::invalid_argument(
                "Requested language is not supported",
            ));
        }

        // Step 4: Query posts filtered by the valid language.
        let query = format!["SELECT * FROM posts WHERE lang.code = '{}'", request_lang];
        println!("[DEBUG] Posts query: {}", query);

        let mut p = DB.query(query).await.map_err(|err| {
            eprintln!("[ERROR] Error querying posts: {:?}", err);
            return Status::internal("Database error fetching posts");
        })?;

        println!("[DEBUG] Posts query executed successfully.");

        let posts_result = p.take(0);
        println!("[DEBUG] Posts query result: {:?}", posts_result);

        let pts: Vec<Option<Post>> = posts_result.ok().unwrap_or_default();
        let mut posts: Vec<PH> = vec![];

        for post in pts {
            if let Some(post) = post {
                posts.push(PH {
                    id: post.id.id.to_string(),
                    title: post.title,
                    description: post.description,
                    lang: Some(Lang {
                        id: post.lang.id.to_string(),
                    }),
                    author: Some(Au {
                        id: post.author.id.to_string(),
                    }),
                    date: post.date,
                });
            }
        }

        // Return the posts in the gRPC response.
        println!("[DEBUG] Sending response with {} posts", posts.len());
        Ok(Response::new(ServePosts { posts }))
    }

    // The following methods are unimplemented for brevity.
    async fn get_post(&self, _request: Request<GetPost>) -> Result<Response<ServePost>, Status> {
        let mut q = DB.query("SELECT id FROM posts").await.map_err(|err| {
            eprint!("ERROR FETCHING POSTS: {:?}", err);
            Status::internal("DATABASE ERROR")
        })?;
        let Ok(ids): Result<Vec<Option<ID>>, Error> = q.take(0) else {
            return Err(Status::aborted("ERROR WHILE READING POSTS"));
        };
        let request_id = _request.into_inner().id;
        if !ids.iter().any(|l| {
            l.as_ref()
                .map(|id| Thing::to_string(&id.id) == format!("posts:{}", request_id))
                .unwrap_or(false)
        }) {
            return Err(Status::internal("DID NOT FIND POST ON DATABASE"));
        }

        let mut p = DB
            .query(format!(
                "SELECT * FROM posts WHERE id = posts:{} ",
                request_id
            ))
            .await
            .map_err(|err| {
                eprint!("ERROR QUERING POSTS: {:?}", err);
                Status::internal("FAILED TO FECTH POST")
            })?;
        let Ok(post): Result<Option<Post>, Error> = p.take(0) else {
            return Err(Status::aborted("FAILED PARSING THE POST"));
        };

        let post = post.unwrap(); // Unwrap the outer Result

        Ok(Response::new(ServePost {
            post: Some(P {
                post_id: post.id.id.to_string(),
                title: post.title,
                description: post.description,
                lang: Some(Lang {
                    id: post.lang.id.to_string(),
                }),
                author: Some(Au {
                    id: post.author.id.to_string(),
                }),
                date: post.date,
                markdown: post.markdown,
                image_id: Some(ImageId {
                    id: post.image_id.id.to_string(),
                }),
            }),
        }))
    }
}
