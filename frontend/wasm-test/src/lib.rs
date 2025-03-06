use std::str::FromStr;
use tonic::{metadata::AsciiMetadataValue, Request};
use tonic_web_wasm_client::Client;
pub use wasm_bindgen_futures::wasm_bindgen::prelude::*;
use web_sys::console;

// Include proto modules
pub mod authxyz {
    tonic::include_proto!("auth");
}
pub mod student_proto {
    tonic::include_proto!("student");
}
pub mod userposts_proto {
    tonic::include_proto!("userposts");
}
pub mod projects_proto {
    tonic::include_proto!("projects");
}

// Import client types from proto modules
use authxyz::{
    authentication_client::AuthenticationClient, AdminLoginRequest, RefreshRequest,
    UserLoginRequest, ValidateTokenRequest,
};
use projects_proto::{
    projects_administration_client::ProjectsAdministrationClient, GProject, GProjects,
};
use student_proto::{
    student_administration_client::StudentAdministrationClient, AddStudent, GetStudent, Student,
};
use userposts_proto::{
    userposts_administration_client::UserpostsAdministrationClient, GetPost, GetPosts,
};

// Backend URL configuration
static URL_BACKEND: &str = "http://127.0.0.1:50051";

// Helper function to build a client.
// Note: Client::new() returns a Client directly.
fn build_client() -> Client {
    Client::new(URL_BACKEND.to_string())
}

// Helper function for error handling
fn grpc_error(error: tonic::Status) -> JsValue {
    console::log_1(&format!("gRPC error: {}", error).into());
    JsValue::from_str(
        &serde_json::json!({
            "code": error.code().to_string(),
            "description": error.message(),
        })
        .to_string(),
    )
}

// Auth token handling for authenticated requests
fn auth_header(token: &str) -> Result<AsciiMetadataValue, tonic::Status> {
    AsciiMetadataValue::from_str(&format!("Bearer {}", token))
        .map_err(|_| tonic::Status::internal("Failed to create auth header"))
}

// ==============================
// AUTH SERVICE IMPLEMENTATIONS
// ==============================

#[wasm_bindgen]
pub async fn admin_login(
    email: String,
    password: String,
    institution: String,
) -> Result<JsValue, JsValue> {
    let mut client = AuthenticationClient::new(build_client());

    let result = client
        .admin_login(AdminLoginRequest {
            email,
            password,
            institution,
        })
        .await;

    match result {
        Ok(response) => {
            let res = response.into_inner();
            Ok(JsValue::from_str(
                &serde_json::json!({
                    "code": "0",
                    "description": {
                        "auth": res.auth_token,
                        "refresh": res.refresh_token,
                    }
                })
                .to_string(),
            ))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn user_login(email: String, password: String) -> Result<JsValue, JsValue> {
    let mut client = AuthenticationClient::new(build_client());

    let result = client
        .user_login(UserLoginRequest { email, password })
        .await;

    match result {
        Ok(response) => {
            let res = response.into_inner();
            Ok(JsValue::from_str(
                &serde_json::json!({
                    "code": "0",
                    "description": {
                        "auth": res.auth_token,
                        "refresh": res.refresh_token,
                    }
                })
                .to_string(),
            ))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn validate_token_admin(token: String) -> Result<JsValue, JsValue> {
    let mut client = AuthenticationClient::new(build_client());

    let result = client
        .validate_token_admin(ValidateTokenRequest { auth_token: token })
        .await;

    match result {
        Ok(_) => Ok(JsValue::from_str(
            &serde_json::json!({
                "code": "0",
                "description": "Token is valid"
            })
            .to_string(),
        )),
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn validate_token_user(token: String) -> Result<JsValue, JsValue> {
    let mut client = AuthenticationClient::new(build_client());

    let result = client
        .validate_token_user(ValidateTokenRequest { auth_token: token })
        .await;

    match result {
        Ok(_) => Ok(JsValue::from_str(
            &serde_json::json!({
                "code": "0",
                "description": "Token is valid"
            })
            .to_string(),
        )),
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn refresh_token_admin(refresh_token: String) -> Result<JsValue, JsValue> {
    let mut client = AuthenticationClient::new(build_client());

    let result = client
        .refresh_token_admin(RefreshRequest { refresh_token })
        .await;

    match result {
        Ok(response) => {
            let res = response.into_inner();
            Ok(JsValue::from_str(
                &serde_json::json!({
                    "code": "0",
                    "description": {
                        "auth": res.auth_token,
                    }
                })
                .to_string(),
            ))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn refresh_token_user(refresh_token: String) -> Result<JsValue, JsValue> {
    let mut client = AuthenticationClient::new(build_client());

    let result = client
        .refresh_token_user(RefreshRequest { refresh_token })
        .await;

    match result {
        Ok(response) => {
            let res = response.into_inner();
            Ok(JsValue::from_str(
                &serde_json::json!({
                    "code": "0",
                    "description": {
                        "auth": res.auth_token,
                    }
                })
                .to_string(),
            ))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

// ==============================
// STUDENT SERVICE IMPLEMENTATIONS
// ==============================

#[wasm_bindgen]
pub async fn add_student(
    token: String,
    name: String,
    email: String,
    student_id: String,
    phone: String,
    info: Option<String>,
) -> Result<JsValue, JsValue> {
    let mut client = StudentAdministrationClient::new(build_client());

    let auth = auth_header(&token).map_err(|e| JsValue::from_str(&format!("Auth error: {}", e)))?;

    let student = Student {
        name,
        email,
        student_id, // generated from studentID in proto
        phone,
        interview: None,
        info_provided_by_user: info, // Option<String> is fine here
    };

    let mut request = Request::new(AddStudent {
        student: Some(student),
    });

    request.metadata_mut().insert("authorization", auth);

    let result = client.add_student(request).await;

    match result {
        Ok(_) => Ok(JsValue::from_str(
            &serde_json::json!({
                "code": "0",
                "description": "Student added successfully"
            })
            .to_string(),
        )),
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn get_student(token: String, student_id: String) -> Result<JsValue, JsValue> {
    let mut client = StudentAdministrationClient::new(build_client());

    let auth = auth_header(&token).map_err(|e| JsValue::from_str(&format!("Auth error: {}", e)))?;

    // The proto message "getStudent" has a field "studentId"
    let mut request = Request::new(GetStudent { student_id });

    request.metadata_mut().insert("authorization", auth);

    let result = client.get_student(request).await;

    match result {
        Ok(response) => {
            let student = response.into_inner();
            Ok(JsValue::from_str(&serde_json::to_string(&student).unwrap()))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn delete_student(token: String, student_id: String) -> Result<JsValue, JsValue> {
    let mut client = StudentAdministrationClient::new(build_client());

    let auth = auth_header(&token).map_err(|e| JsValue::from_str(&format!("Auth error: {}", e)))?;

    // The proto for DeleteStudent expects a getStudent message.
    let mut request = Request::new(GetStudent { student_id });

    request.metadata_mut().insert("authorization", auth);

    let result = client.delete_student(request).await;

    match result {
        Ok(_) => Ok(JsValue::from_str(
            &serde_json::json!({
                "code": "0",
                "description": "Student deleted successfully"
            })
            .to_string(),
        )),
        Err(error) => Err(grpc_error(error)),
    }
}

// ==============================
// USERPOSTS SERVICE IMPLEMENTATIONS
// ==============================

#[wasm_bindgen]
pub async fn get_posts(lang: String) -> Result<JsValue, JsValue> {
    let mut client = UserpostsAdministrationClient::new(build_client());

    let result = client.get_posts(GetPosts { lang }).await;

    match result {
        Ok(response) => {
            let posts = response.into_inner();
            Ok(JsValue::from_str(&serde_json::to_string(&posts).unwrap()))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn get_post(id: String) -> Result<JsValue, JsValue> {
    let mut client = UserpostsAdministrationClient::new(build_client());

    let result = client.get_post(GetPost { id }).await;

    match result {
        Ok(response) => {
            let post = response.into_inner();
            Ok(JsValue::from_str(&serde_json::to_string(&post).unwrap()))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

// ==============================
// PROJECTS SERVICE IMPLEMENTATIONS
// ==============================

#[wasm_bindgen]
pub async fn get_projects(lang: String) -> Result<JsValue, JsValue> {
    let mut client = ProjectsAdministrationClient::new(build_client());

    let result = client.get_projects(GProjects { lang }).await;

    match result {
        Ok(response) => {
            let projects = response.into_inner();
            Ok(JsValue::from_str(
                &serde_json::to_string(&projects).unwrap(),
            ))
        }
        Err(error) => Err(grpc_error(error)),
    }
}

#[wasm_bindgen]
pub async fn get_project(id: String) -> Result<JsValue, JsValue> {
    let mut client = ProjectsAdministrationClient::new(build_client());

    let result = client.get_project(GProject { id }).await;

    match result {
        Ok(response) => {
            let project = response.into_inner();
            Ok(JsValue::from_str(&serde_json::to_string(&project).unwrap()))
        }
        Err(error) => {
            console::log_1(&format!("Get project error: {}", error).into());
            Err(grpc_error(error))
        }
    }
}
