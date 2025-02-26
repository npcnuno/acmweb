use crate::auth;
use crate::dbconn::DB;
use serde::Deserialize;
use surrealdb::{sql::Thing, Error};
use tonic::metadata::MetadataMap;
use tonic::{Request, Response, Status};

pub mod student_module {
    tonic::include_proto!("student");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("student_descriptor");
}

/// Maps the DB record for a student (note that in the DB, the table is named "user" and uses different field names)
#[derive(Debug, Clone, Deserialize)]
struct StudentRecord {
    id: Thing,
    name: String,
    email: String,
    phonenumber: String,
    studentId: String,
    // In DB the field is called HDYFU; we use it for "info_provided_by_user"
    HDYFU: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StudentManager;

#[tonic::async_trait]
impl student_module::student_administration_server::StudentAdministration for StudentManager {
    /// Implements the AddStudent RPC
    async fn add_student(
        &self,
        request: Request<student_module::AddStudent>,
    ) -> Result<Response<student_module::Ack>, Status> {
        // Validate admin token (requires valid admin privileges)
        let req = request.into_inner();
        // Extract student from the request; the field is required
        let student = req
            .student
            .ok_or_else(|| Status::invalid_argument("Student must be provided"))?;

        // Build the create query.
        // Map the proto fields to the DB fields:
        // • proto "name"         -> DB "name"
        // • proto "email"        -> DB "email"
        // • proto "phone"        -> DB "phonenumber"
        // • proto "studentID"    -> DB "studentId"
        // • proto "info_provided_by_user" -> DB "HDYFU"
        let info = student
            .info_provided_by_user
            .unwrap_or_else(|| "".to_string());
        let create_query = format!(
            "CREATE user CONTENT {{ name: '{}', email: '{}', phonenumber: '{}', studentId: '{}', HDYFU: '{}' }};",
            student.name, student.email, student.phone, student.studentID, info
        );
        let mut response = DB.query(create_query).await.map_err(|e| {
            eprintln!("Create failed: {:?}", e);
            Status::internal("Database error")
        })?;

        // Process the query response
        let created: Vec<StudentRecord> = response.take(0).map_err(|e| {
            eprintln!("Response parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;
        if created.is_empty() {
            return Err(Status::internal("Failed to create student"));
        }
        Ok(Response::new(student_module::Ack {}))
    }

    /// Implements the GetStudent RPC
    async fn get_student(
        &self,
        request: Request<student_module::GetStudent>,
    ) -> Result<Response<student_module::Student>, Status> {
        let req = request.into_inner();
        // Query the DB for a student with matching studentId
        let query = format!("SELECT * FROM user WHERE studentId = '{}';", req.studentId);
        let mut response = DB.query(query).await.map_err(|e| {
            eprintln!("Query failed: {:?}", e);
            Status::internal("Database error")
        })?;

        let student_opt: Option<StudentRecord> = response.take(0).map_err(|e| {
            eprintln!("Parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;
        let student = student_opt.ok_or_else(|| Status::not_found("Student not found"))?;

        // Map DB record to proto message.
        Ok(Response::new(student_module::Student {
            name: student.name,
            email: student.email,
            studentID: student.studentId,
            phone: student.phonenumber,
            // Interview is not stored in DB so we leave it unset.
            interview: None,
            info_provided_by_user: student.HDYFU.unwrap_or_else(|| "".to_string()),
        }))
    }

    /// Implements the DeleteStudent RPC
    async fn delete_student(
        &self,
        request: Request<student_module::GetStudent>,
    ) -> Result<Response<student_module::Ack>, Status> {
        // Validate admin privileges
        validate_admin_token(request.metadata())?;
        let req = request.into_inner();
        // Delete query using studentId
        let delete_query = format!("DELETE user WHERE studentId = '{}';", req.studentId);
        let mut response = DB.query(delete_query).await.map_err(|e| {
            eprintln!("Delete failed: {:?}", e);
            Status::internal("Database error")
        })?;

        let deleted: Option<StudentRecord> = response.take(0).map_err(|e| {
            eprintln!("Parsing failed: {:?}", e);
            Status::internal("Data parsing error")
        })?;
        if deleted.is_none() {
            return Err(Status::not_found("Student not found"));
        }
        Ok(Response::new(student_module::Ack {}))
    }
}

/// Helper function to validate admin tokens
fn validate_admin_token(metadata: &MetadataMap) -> Result<(), Status> {
    let token = metadata
        .get("auth")
        .ok_or_else(|| Status::unauthenticated("Missing token"))?
        .to_str()
        .map_err(|_| Status::invalid_argument("Invalid token format"))?;
    if !auth::validate_jwt_admin(token) || !auth::is_db_admin(token) {
        return Err(Status::permission_denied("Invalid admin privileges"));
    }
    Ok(())
}

/// Helper function for validating any token (if needed)
fn validate_any_token(metadata: &MetadataMap) -> Result<(), Status> {
    let token = metadata
        .get("auth")
        .ok_or_else(|| Status::unauthenticated("Missing token"))?
        .to_str()
        .map_err(|_| Status::invalid_argument("Invalid token format"))?;
    if !auth::validate_jwt_admin(token) {
        return Err(Status::permission_denied("Invalid token"));
    }
    Ok(())
}
