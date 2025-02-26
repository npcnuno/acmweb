use crate::dbconn::DB;
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Error};
use tonic::{Request, Response, Status};

pub mod projects_module {
    tonic::include_proto!("projects");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("projects_descriptor");
}

use self::projects_module::{
    Author, GProject, GProjects, ImageId, Lang, ProgLang, Progress, Project, ProjectHeader,
    ServeProject, ServeProjects,
};

#[derive(Debug, Clone, Deserialize)]
struct ProjectRecord {
    id: Thing,
    name: String,
    lang: Thing,
    status: Thing,
    authors: Vec<Thing>,
    image_id: Thing,
    prog_lang: Vec<Thing>,
}

#[derive(Debug, Clone, Deserialize)]
struct LanguageRecord {
    id: Thing,
    code: String,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectsManager;

#[tonic::async_trait]
impl projects_module::projects_administration_server::ProjectsAdministration for ProjectsManager {
    async fn get_projects(
        &self,
        request: Request<GProjects>,
    ) -> Result<Response<ServeProjects>, Status> {
        let lang_code = request.into_inner().lang;

        // Validate language code
        let mut lang_query = DB
            .query(format!(
                "SELECT * FROM languages WHERE code = '{}'",
                lang_code
            ))
            .await
            .map_err(|e| {
                eprintln!("Language query error: {e:?}");
                Status::internal("Database error")
            })?;

        let lang_exists: Option<LanguageRecord> = lang_query.take(0).map_err(|e| {
            eprintln!("Language parsing error: {e:?}");
            Status::internal("Data parsing error")
        })?;

        if lang_exists.is_none() {
            return Err(Status::invalid_argument("Invalid language code"));
        }

        // Get projects for language
        let mut query = DB
            .query(format!(
                "SELECT * FROM projects WHERE lang.code = '{}'",
                lang_code
            ))
            .await
            .map_err(|e| {
                eprintln!("Projects query error: {e:?}");
                Status::internal("Database error")
            })?;

        let projects: Vec<ProjectRecord> = query.take(0).map_err(|e| {
            eprintln!("Projects parsing error: {e:?}");
            Status::internal("Data parsing error")
        })?;

        let headers = projects
            .into_iter()
            .map(|p| ProjectHeader {
                id: p.id.id.to_string(),
                name: p.name,
                lang: Some(Lang {
                    id: p.lang.id.to_string(),
                }),
                status: Some(Progress {
                    id: p.status.id.to_string(),
                }),
            })
            .collect();

        Ok(Response::new(ServeProjects { projects: headers }))
    }

    async fn get_project(
        &self,
        request: Request<GProject>,
    ) -> Result<Response<ServeProject>, Status> {
        let project_id = request.into_inner().id;

        let mut query = DB
            .query(format!(
                "SELECT * FROM projects WHERE id = projects:{}",
                project_id
            ))
            .await
            .map_err(|e| {
                eprintln!("Project query error: {e:?}");
                Status::internal("Database error")
            })?;

        let project: Option<ProjectRecord> = query.take(0).map_err(|e| {
            eprintln!("Project parsing error: {e:?}");
            Status::internal("Data parsing error")
        })?;

        let project = project.ok_or_else(|| Status::not_found("Project not found"))?;

        Ok(Response::new(ServeProject {
            project: Some(Project {
                id: project.id.id.to_string(),
                name: project.name,
                lang: Some(Lang {
                    id: project.lang.id.to_string(),
                }),
                status: Some(Progress {
                    id: project.status.id.to_string(),
                }),
                authors: project
                    .authors
                    .into_iter()
                    .map(|a| Author {
                        id: a.id.to_string(),
                    })
                    .collect(),
                image_id: Some(ImageId {
                    id: project.image_id.id.to_string(),
                }),
                prog_lang: project
                    .prog_lang
                    .into_iter()
                    .map(|p| ProgLang {
                        id: p.id.to_string(),
                    })
                    .collect(),
            }),
        }))
    }
}
