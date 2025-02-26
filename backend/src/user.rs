use std::pin::Pin;

use crate::auth;
use crate::dbconn;
use dbconn::DB;
use rand::Rng;
use serde::Deserialize;
use surrealdb::{sql::Thing, Error};
use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

use user_module::{
    user_administration_server::UserAdministration, Ack, CreateUserRequest, DeleteUserRequest,
    DownloadImageRequest, DownloadImageResponse, GetMultipleUsersRequest, GetMultipleUsersResponse,
    GetUserRequest, GetUserResponse, RequestUserPasswordResetRequest, ResetUserPasswordRequest,
    UpdateUserPasswordRequest, UpdateUserRequest, UploadImageRequest, User, UserId,
    ValidateResetTokenRequest,
};

pub mod user_module {
    use tonic::include_proto;
    include_proto!("user");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("user_descriptor");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Aluno {
    id: Thing,
    name: String,
    email: String,
    phone: String,
    pub institution: Thing,
    is_partner: bool,
    first_time: Option<bool>,
    role: String,
}

// macro_rules! exec_task {
//     ($($body:tt)*) => {
//         let tr = tokio::task::spawn(async move{
//             $($body)*
//         });
//         let tr = match tr.await{
//             Ok(a) => a,
//             Err(_) => Err(Status::internal("")),
//         };
//         return tr;
//     };
// }

const IMAGES_PATH: &str = "images/user/";

#[derive(Default, Clone)]
pub struct UserManager {}

#[tonic::async_trait]
impl UserAdministration for UserManager {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("Auth Invalid")),
        };
        let token = auth.to_str().unwrap().to_string();
        let user_id = request.into_inner().id.unwrap().id;
        // verify if is admin or if it is user (user can only get themself)
        if !auth::validate_jwt_admin(&token) {
            if auth::validate_jwt_user(&token) {
                let (_, claims) = match auth::decode_jwt(&token) {
                    Ok(a) => a,
                    Err(_) => return Err(Status::permission_denied("id's don't match")),
                };
                if !claims.uid.eq(user_id.as_str()) {
                    return Err(Status::permission_denied("id's don't match"));
                }
            } else {
                return Err(Status::permission_denied("token not valid"));
            }
        }

        // db connection

        let user = DB.query(format!("select * from user:{}", user_id)).await;
        // result from query
        let res: Result<Option<Aluno>, Error> = user.unwrap().take(0);
        //check if record is parsed
        let res = match res {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Parsing failed : {}", e);
                return Err(Status::aborted("Record not well structured in db"));
            }
        };
        //check if user exists
        let res = match res {
            Some(b) => b,
            None => return Err(Status::aborted("User not found")),
        };
        let reply = User {
            id: Some(UserId {
                id: res.id.id.to_string(),
            }),
            name: res.name,
            email: res.email.clone(),
            phone: res.phone,
            is_partner: res.is_partner,
            first_time: res.first_time.unwrap_or(false),
            institution: res.institution.id.to_string(),
            role: res.role,
        };

        tracing::info!(message="get_user - ", %res.email);
        Ok(Response::new(GetUserResponse { user: Some(reply) }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let ins_tok: Option<String> = match auth {
            Some(a) => {
                if !auth::validate_jwt_admin(a.to_str().unwrap()) {
                    return Err(Status::permission_denied("token not valid"));
                }
                let (_, dec) = auth::decode_jwt(a.to_str().unwrap()).unwrap();
                match dec.ins.as_str() {
                    "ADMIN" => None,
                    _ => Some(dec.ins),
                }
            }
            None => Some(String::from("NONE")),
        };
        let req = request.into_inner();
        let rr = Aluno {
            id: Thing {
                tb: "".to_string(),
                id: surrealdb::sql::Id::String("".to_string()),
            },
            name: req.name.unwrap_or_default(),
            email: req.email,
            phone: req.phone.unwrap_or_default(),
            first_time: Some(true),
            is_partner: req.is_partner.unwrap_or(false),
            institution: Thing {
                tb: "institution".to_string(),
                id: surrealdb::sql::Id::String(
                    ins_tok.unwrap_or(req.institution.unwrap_or(String::from("NONE"))),
                ),
            },
            role: req.role.unwrap_or("student".to_string()),
        };
        let random_string = rand_string();
        if !check_email(rr.email.clone()).await {
            return Err(Status::aborted("Email invalid"));
        }
        if !check_exists_email(rr.email.clone()).await {
            if check_user_first_time(rr.email.clone()).await {
                let _ = DB.query("BEGIN TRANSACTION;").await;
                let _ = DB
                .query(format!("update user where (email=\"{}\") set password=crypto::argon2::generate(\"{}\")", &rr.email, random_string))
                .await;
                match auth::send_email(
                    &rr.email,
                    "Your temporary password for AEWallet",
                    &format!("Your temp pass for aewallet is: {}", random_string),
                )
                .await
                {
                    Ok(_) => {
                        let _ = DB.query("COMMIT TRANSACTION;").await;
                        return Err(Status::ok("Created user again"));
                    }
                    Err(_) => {
                        return Err(Status::already_exists(""));
                    }
                }
            }
            return Err(Status::aborted("User already exists"));
        }
        if !check_institution(rr.institution.id.to_string().clone()).await {
            return Err(Status::aborted("Institution not supported"));
        }
        let _ = DB.query("BEGIN TRANSACTION;").await;
        println!("\"{}\"", rr.role);
        let qq = DB
            .query(format!(
                "create user set
                    name=\"{}\",
                    email=\"{}\",
                    phone=\"{}\",
                    image=\"\",
                    password=crypto::argon2::generate(\"{}\"),
                    first_time=true,
                    is_partner={},
                    institution=institution:{},
                    role=\"{}\";
                    ",
                rr.name,
                rr.email,
                rr.phone,
                random_string,
                rr.is_partner,
                rr.institution.id,
                rr.role,
            ))
            .await;
        match qq {
            Ok(_) => {}
            Err(_) => {
                let _ = DB.query("CANCEL TRANSACTION;").await;
                return Err(Status::aborted("couldn't create user"));
            }
        };
        tracing::debug!("temp password of user {}:{}", rr.email, random_string);
        match auth::send_email(
            &rr.email,
            "Your temporary password for AEWallet",
            &format!("Your temp pass for aewallet is: {}", random_string),
        )
        .await
        {
            Ok(_) => {
                let _ = DB.query("COMMIT TRANSACTION;").await;
                return Ok(Response::new(Ack {}));
            }
            Err(_) => {
                let _ = DB.query("CANCEL TRANSACTION;").await;
                return Err(Status::cancelled("couldnt send email"));
            }
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = match request.metadata().get("auth") {
            Some(a) => a.clone(),
            None => return Err(Status::aborted("no token")),
        };

        let req = request.into_inner().clone();
        if auth::validate_jwt_user(auth.to_str().unwrap()) {
            let (_, c) = match auth::decode_jwt(auth.to_str().unwrap()) {
                Ok(a) => a,
                Err(_) => return Err(Status::permission_denied("id's don't match")),
            };

            if !c.uid.eq(req.id.clone().unwrap().id.as_str()) {
                return Err(Status::permission_denied("id's don't match"));
            }
        } else if !auth::validate_jwt_admin(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("id's don't match"));
        }

        // let req = request.into_inner();
        let mut st = String::from("");
        if let Some(a) = req.email {
            st = format!("{} email=\"{}\",", st, a);
        };
        if let Some(a) = req.name {
            st = format!("{} name=\"{}\",", st, a);
        };
        if let Some(a) = req.phone {
            st = format!("{} phone=\"{}\",", st, a);
        };
        if st.is_empty() {
            return Ok(Response::new(Ack {}));
        }
        let _ = st.pop();
        st.push(';');
        let q = DB
            .query(format!("update user:{} set {}", req.id.unwrap().id, st))
            .await;
        match q {
            Ok(_) => Ok(Response::new(Ack {})),
            Err(_) => Err(Status::internal("failed to query the db")),
        }
    }

    async fn update_user_password(
        &self,
        request: Request<UpdateUserPasswordRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_user(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("token not valid"));
        }
        let (_, token) = auth::decode_jwt(auth.to_str().unwrap()).unwrap();
        let pass = request.into_inner().password;
        let q = DB
            .query(format!(
                "update user:{} set first_time=NONE,password=crypto::argon2::generate(\"{}\")",
                token.uid, pass
            ))
            .await;
        if q.is_ok() {
            return Ok(Response::new(Ack {}));
        }
        return Err(Status::unimplemented(""));
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_admin(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("token not valid"));
        }
        let req = request.into_inner();

        let query = DB
            .query(format!("delete user:{}", req.id.clone().unwrap().id))
            .await;
        match query {
            Ok(_) => {
                tracing::info!("User deleted: {}", req.id.unwrap().id);
                return Ok(Response::new(Ack {}));
            }
            Err(_) => return Err(Status::not_found("")),
        };
    }

    async fn get_multiple_users(
        &self,
        request: Request<GetMultipleUsersRequest>,
    ) -> Result<Response<GetMultipleUsersResponse>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        let token = auth.to_str().unwrap().to_string();
        if !auth::validate_jwt_admin(&token) {
            return Err(Status::permission_denied("token not valid"));
        }
        let ins = auth::decode_jwt(&token).unwrap().1.ins;
        let ins = match ins.as_str() {
            "ADMIN" => "where (1=1)".to_string(),
            _ => format!("where institution=institution:{}", ins),
        };
        let req = request.into_inner();
        let start = req.start as usize;
        let limit = req.limit as usize;
        let search = req.search;
        let search = match search {
            Some(a) => format!("and (name ~ \"{a}\" or phone ~ \"{a}\" or email ~ \"{a}\") "),
            None => String::from("and (1=1)"),
        };
        println!("{}", search);

        let sel = DB
            .query(format!(
                "select * from user {} {} limit {} start {};",
                ins, search, limit, start
            ))
            .await;
        println!(
            "select * from user {} {} limit {} start {};",
            ins, search, limit, start
        );
        let mut sel = match sel {
            Ok(a) => a,
            Err(_) => return Err(Status::not_found("query empty")),
        };

        let sel: Result<Vec<Option<Aluno>>, Error> = sel.take(0);
        let sel = match sel {
            Ok(a) => a,
            Err(e) => {
                tracing::error!("{e}");
                return Err(Status::not_found(format!("no user {}", e)));
            }
        };
        let sel: Vec<_> = sel.iter().map(|x| x.as_ref().unwrap()).collect();

        let mut users: Vec<User> = vec![];
        for al in sel {
            users.push(User {
                id: Some(UserId {
                    id: al.id.id.to_string(),
                }),
                name: al.name.clone(),
                email: al.email.clone(),
                phone: al.phone.clone(),
                first_time: al.first_time.unwrap_or(false),
                is_partner: al.is_partner,
                institution: al.institution.id.to_string(),
                role: al.role.clone(),
            });
        }
        return Ok(Response::new(GetMultipleUsersResponse { users }));
    }

    async fn upload_user_image(
        &self,
        request: Request<Streaming<UploadImageRequest>>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_user(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("token not valid"));
        }
        let user_id = auth::decode_jwt(auth.to_str().unwrap()).unwrap().1.uid;
        let mut stream = request.into_inner();

        let mut file = Vec::<u8>::new();
        let mut file_ext = String::new();
        while let Some(byte_chunk) = stream.message().await.unwrap() {
            file.extend_from_slice(&byte_chunk.image);
            if file_ext.is_empty() {
                file_ext = byte_chunk.ext;
            }
        }
        // file
        let mut file_name = Uuid::new_v4().to_string();
        file_name.push_str(format!(".{}", file_ext).as_str());
        let f = File::create(IMAGES_PATH.to_string() + &file_name).await;
        let mut f = match f {
            Ok(a) => a,
            Err(_) => return Err(Status::internal("file could not be created")),
        };
        if f.write_all(&file).await.is_err() {
            return Err(Status::internal("file could not be written"));
        };
        //
        let old = DB
            .query(format!(
                "return (select image from only user:{}).image",
                user_id
            ))
            .await;
        let old: Result<Option<String>, Error> = old.unwrap().take(0);
        let _ = DB.query("BEGIN TRANSACTION;").await;
        let _ = DB
            .query(format!(
                "update user:{} set image=\"{}\"",
                user_id, file_name
            ))
            .await;

        let old = match old {
            Ok(a) => a,
            Err(_) => {
                let _ = DB.query("CANCEL TRANSACTION;").await;
                let _ = tokio::fs::remove_file(IMAGES_PATH.to_string() + &file_name).await;
                return Err(Status::internal("couldn't get old file from db"));
            }
        };
        if let Some(a) = old {
            let q = tokio::fs::remove_file(IMAGES_PATH.to_string() + &a).await;
            match q {
                Ok(_) => {
                    let _ = DB.query("COMMIT TRANSACTION;").await;
                    return Ok(Response::new(Ack {}));
                }
                Err(e) => {
                    tracing::error!("COULDN't REMOVE IMAGE FILE, {}", e.kind());
                    match e.kind() {
                        std::io::ErrorKind::NotFound | std::io::ErrorKind::Interrupted => {
                            let _ = DB.query("COMMIT TRANSACTION;").await;
                            return Ok(Response::new(Ack {}));
                        }
                        _ => return Err(Status::internal("error removing old image file")),
                    }
                }
            };
        }
        let _ = DB.query("COMMIT TRANSACTION;").await;
        return Ok(Response::new(Ack {}));
    }

    type DownloadUserImageStream =
        Pin<Box<dyn Stream<Item = Result<DownloadImageResponse, Status>> + Send>>;

    async fn download_user_image(
        &self,
        request: Request<DownloadImageRequest>,
    ) -> Result<Response<Self::DownloadUserImageStream>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_user(auth.to_str().unwrap())
            && !auth::validate_jwt_admin(auth.to_str().unwrap())
        {
            return Err(Status::permission_denied("token not valid"));
        }
        let user_id = auth::decode_jwt(auth.to_str().unwrap()).unwrap().1.uid;
        let img = DB
            .query(format!(
                "return (select image from only user:{}).image",
                user_id
            ))
            .await;
        let img: Result<Option<String>, Error> = img.unwrap().take(0);
        let img = match img {
            Ok(a) => a,
            Err(_) => return Err(Status::internal("error in db")),
        };
        let img = match img {
            Some(a) => a,
            None => return Err(Status::internal("image not defined")),
        };
        let file = tokio::fs::read(IMAGES_PATH.to_string() + &img).await;
        let file = match file {
            Ok(a) => a,
            Err(_) => return Err(Status::internal("couldn't read file")),
        };
        let (sender, receiver) = mpsc::channel(32); // Adjust buffer size as needed
        tokio::spawn(async move {
            for chunk in file.chunks(4096) {
                let image_bytes = DownloadImageResponse {
                    image: chunk.to_vec(), // Assuming 'data' field in ImageBytes message is bytes
                };
                if sender.send(Ok(image_bytes)).await.is_err() {
                    // Handle sender error, if any
                    break;
                }
            }
        });
        let r = ReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(r) as Self::DownloadUserImageStream))
    }

    async fn request_user_password_reset(
        &self,
        request: Request<RequestUserPasswordResetRequest>,
    ) -> Result<Response<Ack>, Status> {
        let req = request.into_inner();
        // check if string is email
        if let Ok(Some(q)) = DB
            .query(format!("RETURN string::is::email(\"{}\");", &req.email))
            .await
            .unwrap()
            .take(0)
        {
            match q {
                true => true,
                false => return Err(Status::invalid_argument("not an email")),
            };
        } else {
            return Err(Status::invalid_argument("not an email"));
        }
        // get user id and send reset url to his email
        let id = DB
            .query(format!(
                "return (select id from user where email=\"{}\").id",
                req.email
            ))
            .await;
        let id: Result<Option<String>, Error> = id.unwrap().take(0);
        let id = match id {
            Ok(id) => match id {
                Some(id) => id,
                None => return Err(Status::invalid_argument("not a valid email")),
            },
            Err(_) => return Err(Status::invalid_argument("not a valid email")),
        };
        let reset_jwt = auth::create_jwt_reset(&id, auth::Privilege::USER).await;
        let reset_jwt = match reset_jwt {
            Ok(reset_jwt) => reset_jwt,
            Err(_) => return Err(Status::aborted("Couldn't generate token")),
        };
        if let Ok(_a) = auth::send_email(
            &req.email,
            "AEWallet - Reset Password",
            &format!(
                "We received a request for a password rest for your account.
                    <br> Please click the link bellow within the next 10 minutes to reset your password.
                    <br> <b> <a href=\"https://reset.orizuro.eu/{}\">HTML tutorial</a><b>",
                &reset_jwt,
            )
        )
        .await
        {
            tracing::info!(message="password_reset - Email sent", %req.email);
            return Ok(Response::new(Ack {}));
        }
        return Err(Status::internal("internal server error"));
    }

    async fn reset_user_password(
        &self,
        request: Request<ResetUserPasswordRequest>,
    ) -> Result<Response<Ack>, Status> {
        let reset_token = request.metadata().get("auth");
        let reset_token = match reset_token {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_reset(reset_token.to_str().unwrap()).await {
            return Err(Status::permission_denied("token not valid"));
        }
        let (header, claims) = match auth::decode_reset(reset_token.to_str().unwrap()).await {
            Ok(t) => t,
            Err(_) => return Err(Status::internal("can't decode token")),
        };
        let password = request.into_inner().password;
        // Effetuate password reset
        let _ = DB.query("BEGIN TRANSACTION;").await;
        let q = DB
            .query(format!(
                "update user:{} set first_time=NONE,password=crypto::argon2::generate(\"{}\")",
                claims.uid, password
            ))
            .await;
        if q.is_ok() && auth::invalidate_jwt_reset((header, claims)).await {
            let _ = DB.query("COMMIT TRANSACTION;").await;
            return Ok(Response::new(Ack {}));
        }
        let _ = DB.query("CANCEL TRANSACTION;").await;
        return Err(Status::internal("database error"));
    }

    async fn validate_reset_token(
        &self,
        request: Request<ValidateResetTokenRequest>,
    ) -> Result<Response<Ack>, Status> {
        let reset_token = request.into_inner().token;
        if auth::validate_jwt_reset(&reset_token).await {
            return Ok(Response::new(Ack {}));
        }
        return Err(Status::invalid_argument("invalid token"));
    }
}

//NOTE: for use by web_server; when modules are containerized delete this and use grpc to
//communicate between modules
pub async fn reset_user_password(password: String, token: String) -> Result<Response<Ack>, Status> {
    let reset_token = token.as_str();
    if !auth::validate_jwt_reset(reset_token).await {
        return Err(Status::permission_denied("token not valid"));
    }
    let (header, claims) = match auth::decode_reset(reset_token).await {
        Ok(t) => t,
        Err(_) => return Err(Status::internal("can't decode token")),
    };
    // Effetuate password reset
    let _ = DB.query("BEGIN TRANSACTION;").await;
    let q = DB
        .query(format!(
            "update user:{} set first_time=NONE,password=crypto::argon2::generate(\"{}\")",
            claims.uid, password
        ))
        .await;
    if q.is_ok() && auth::invalidate_jwt_reset((header, claims)).await {
        let _ = DB.query("COMMIT TRANSACTION;").await;
        return Ok(Response::new(Ack {}));
    }
    let _ = DB.query("CANCEL TRANSACTION;").await;
    Err(Status::internal("database error"))
}

// checks if institution exists
// NOTE: Alex o ::<Vec<String>> é uma cena chamada turbo fish serve para dizer o que a função deve retornar quando ela é genérica
async fn check_institution(institution: String) -> bool {
    match DB
        .query(format!("select * from institution:{}", institution))
        .await
    {
        Ok(mut a) => a.take::<Vec<String>>(0).is_err(),
        Err(_) => false,
    }
}
// checks if user exists
async fn check_email(email: String) -> bool {
    let q: Result<Option<bool>, Error> = DB
        .query(format!("RETURN string::is::email(\"{}\");", &email))
        .await
        .unwrap()
        .take(0);
    if q.is_err() {
        return false;
    }
    if q.as_ref().unwrap().is_none() {
        return false;
    }
    if !q.unwrap().unwrap() {
        return false;
    }
    true
}

async fn check_exists_email(email: String) -> bool {
    match DB
        .query(format!("select email from user where email=\"{}\"", email))
        .await
    {
        Ok(mut a) => a.take::<Vec<String>>(0).is_ok(),
        Err(_) => false,
    }
}

async fn check_user_first_time(email: String) -> bool {
    match DB
        .query(format!(
            "select first_time from user where email=\"{}\"",
            email
        ))
        .await
    {
        Ok(mut a) => a.take::<Vec<bool>>(0).is_ok(),
        Err(_) => false,
    }
}

fn rand_string() -> String {
    (0..10)
        .map(|_| -> String {
            let a: char = match rand::thread_rng().gen_range(0..4) {
                0 => rand::thread_rng().gen_range('0'..='9'),
                1 => rand::thread_rng().gen_range('a'..='z'),
                2 => rand::thread_rng().gen_range('A'..='Z'),
                _ => rand::thread_rng().gen_range('#'..='&'),
            };
            a.to_string()
        })
        .collect()
}
