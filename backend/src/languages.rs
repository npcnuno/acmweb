use crate::auth;
use crate::dbconn::DB;
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Error};
use tonic::metadata::MetadataMap;
use tonic::{Request, Response, Status};

pub mod languages_module {
    tonic::include_proto!("languages");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("languages_descriptor");
}

#[derive(Debug, Clone, Deserialize)]
struct LanguageRecord {
    id: Thing,
    code: String,
}

#[derive(Debug, Clone, Default)]
pub struct LanguagesManager;

#[tonic::async_trait]
impl languages_module::languages_administration_server::LanguagesAdministration
    for LanguagesManager
{
    async fn create_language(
        &self,
        request: Request<languages_module::CreateLanguageRequest>,
    ) -> Result<Response<languages_module::CreateLanguageResponse>, Status> {
        // Authentication and validation
        validate_admin_token(request.metadata())?;
        let req = request.into_inner();
        let request_lang = req
            .language
            .ok_or(Status::invalid_argument("Language must be provided"))?;

        if request_lang.code.is_empty() || request_lang.code.len() != 2 {
            return Err(Status::invalid_argument(
                "Language code must be exactly 2 characters long",
            ));
        }

        // Direct query (replacing the transaction block)
        let create_query = format!("CREATE languages SET code = '{}';", request_lang.code);
        let mut create_response = DB.query(create_query).await.map_err(|e| {
            eprintln!("Create failed: {:?}", e);
            Status::internal("Database error")
        })?;

        // Process response
        let created: Vec<LanguageRecord> = create_response.take(0).map_err(|e| {
            eprintln!("Response parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;

        let lang = created
            .first()
            .ok_or(Status::internal("No language created"))?;

        Ok(Response::new(languages_module::CreateLanguageResponse {
            language: Some(languages_module::Language {
                id: lang.id.id.to_string(),
                code: lang.code.clone(),
            }),
        }))
    }

    async fn get_language(
        &self,
        request: Request<languages_module::GetLanguageRequest>,
    ) -> Result<Response<languages_module::GetLanguageResponse>, Status> {
        // Authentication
        let id = request.into_inner().id;

        // Direct query
        let query = format!("SELECT * FROM languages WHERE  code = {};", id);
        let mut response = DB.query(query).await.map_err(|e| {
            eprintln!("Query failed: {:?}", e);
            Status::internal("Database error")
        })?;

        let lang: Option<LanguageRecord> = response.take(0).map_err(|e| {
            eprintln!("Parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;

        let lang = lang.ok_or(Status::not_found("Language not found"))?;

        Ok(Response::new(languages_module::GetLanguageResponse {
            language: Some(languages_module::Language {
                id: lang.id.id.to_string(),
                code: lang.code,
            }),
        }))
    }

    async fn update_language(
        &self,
        request: Request<languages_module::UpdateLanguageRequest>,
    ) -> Result<Response<languages_module::UpdateLanguageResponse>, Status> {
        // Authentication and validation
        validate_admin_token(request.metadata())?;
        let req = request.into_inner();
        let request_lang = req
            .language
            .ok_or(Status::invalid_argument("Language must be provided"))?;

        if request_lang.code.is_empty() || request_lang.code.len() != 2 {
            return Err(Status::invalid_argument(
                "Language code must be exactly 2 characters long",
            ));
        }

        // Direct query (replacing the transaction block)
        let update_query = format!(
            "UPDATE {} SET code = '{}';",
            request_lang.id, request_lang.code
        );
        let mut update_response = DB.query(update_query).await.map_err(|e| {
            eprintln!("Update failed: {:?}", e);
            Status::internal("Database error")
        })?;

        // Process response
        let updated: Option<LanguageRecord> = update_response.take(0).map_err(|e| {
            eprintln!("Parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;

        let updated = updated.ok_or(Status::not_found("Language not found"))?;

        Ok(Response::new(languages_module::UpdateLanguageResponse {
            language: Some(languages_module::Language {
                id: updated.id.id.to_string(),
                code: updated.code,
            }),
        }))
    }

    async fn delete_language(
        &self,
        request: Request<languages_module::DeleteLanguageRequest>,
    ) -> Result<Response<languages_module::DeleteLanguageResponse>, Status> {
        // Authentication
        validate_admin_token(request.metadata())?;
        let id = request.into_inner().id;

        // Direct query (replacing the transaction block)
        let delete_query = format!("DELETE {};", id);
        let mut delete_response = DB.query(delete_query).await.map_err(|e| {
            eprintln!("Delete failed: {:?}", e);
            Status::internal("Database error")
        })?;

        // Process response
        let deleted: Option<LanguageRecord> = delete_response.take(0).map_err(|e| {
            eprintln!("Parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;

        Ok(Response::new(languages_module::DeleteLanguageResponse {
            success: deleted.is_some(),
        }))
    }

    async fn list_languages(
        &self,
        request: Request<languages_module::ListLanguagesRequest>,
    ) -> Result<Response<languages_module::ListLanguagesResponse>, Status> {
        // Authentication
        validate_any_token(request.metadata())?;
        let req = request.into_inner();

        // Since page and page_size are i32 values (not Options),
        // use default values if they are 0.
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let start = (page - 1) * page_size;

        // Query with pagination
        let mut query = DB
            .query("SELECT * FROM languages LIMIT $page_size START $start")
            .bind(("page_size", page_size))
            .bind(("start", start))
            .await
            .map_err(|e| {
                eprintln!("Query failed: {:?}", e);
                Status::internal("Database error")
            })?;

        let languages: Vec<LanguageRecord> = query.take(0).map_err(|e| {
            eprintln!("Parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;

        Ok(Response::new(languages_module::ListLanguagesResponse {
            languages: languages
                .into_iter()
                .map(|lang| languages_module::Language {
                    id: lang.id.id.to_string(),
                    code: lang.code,
                })
                .collect(),
        }))
    }
}

// Authentication helper functions
fn validate_admin_token(metadata: &MetadataMap) -> Result<(), Status> {
    let token = metadata
        .get("auth")
        .ok_or(Status::unauthenticated("Missing token"))?
        .to_str()
        .map_err(|_| Status::invalid_argument("Invalid token format"))?;

    if !auth::validate_jwt_admin(token) || !auth::is_db_admin(token) {
        return Err(Status::permission_denied("Invalid admin privileges"));
    }

    Ok(())
}

fn validate_any_token(metadata: &MetadataMap) -> Result<(), Status> {
    let token = metadata
        .get("auth")
        .ok_or(Status::unauthenticated("Missing token"))?
        .to_str()
        .map_err(|_| Status::invalid_argument("Invalid token format"))?;

    if !auth::validate_jwt_admin(token) {
        return Err(Status::permission_denied("Invalid token"));
    }

    Ok(())
}
