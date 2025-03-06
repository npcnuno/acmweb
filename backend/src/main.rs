use auth::AuthManager;
use projects::ProjectsManager;
use tokio::time::Duration;
use tonic::{transport::Server, Request, Response, Status};
use userposts::UserpostsManager;
pub mod auth;
pub mod dbconn;
pub mod languages;
pub mod projects;
pub mod student;
pub mod userposts;
use dbconn::DB;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};
pub mod hello_world {
    tonic::include_proto!("helloworld");
}
use crate::auth::auth_module;
use crate::languages::languages_module;
use crate::projects::projects_module;
use crate::student::student_module;
use crate::userposts::userposts_module;
use auth_module::authentication_server::AuthenticationServer;
use languages::LanguagesManager;
use languages_module::languages_administration_server::LanguagesAdministrationServer;
use projects_module::projects_administration_server::ProjectsAdministrationServer;
use student::StudentManager;
use student_module::student_administration_server::StudentAdministrationServer;
use tonic_web::{CorsGrpcWeb, GrpcWebLayer};
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};
use userposts_module::userposts_administration_server::UserpostsAdministrationServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    tokio::time::sleep(Duration::from_secs(15)).await;
    let db_url = match std::env::var("DB_URL") {
        Ok(a) => a,
        Err(_) => panic!("$DB_URL not defined"),
    };
    DB.connect::<Ws>(db_url).await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    DB.use_ns("teste").use_db("testeprod").await?;
    dbconn::create_file_system().await;
    dbconn::define_db_functions().await;

    // Define CORS settings
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_headers(AllowHeaders::any())
        .allow_origin(AllowOrigin::any());

    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_headers(AllowHeaders::any())
                .allow_origin(AllowOrigin::any()),
        )
        .layer(GrpcWebLayer::new())
        .add_service(AuthenticationServer::new(AuthManager::default()))
        .add_service(UserpostsAdministrationServer::new(
            UserpostsManager::default(),
        ))
        .add_service(ProjectsAdministrationServer::new(ProjectsManager::default()))
        .add_service(LanguagesAdministrationServer::new(
            LanguagesManager::default(),
        ))
        .add_service(StudentAdministrationServer::new(StudentManager::default()))
        .serve(addr)
        .await?;

    Ok(())
}
