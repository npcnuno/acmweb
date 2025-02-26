use once_cell::sync::Lazy;
use redis::{AsyncCommands, Client as Rc};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::fs;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn create_file_system() {
    // Create directories if they don't exist
    let dirs = ["images/user", "images/posts"];
    for dir in dirs.iter() {
        if let Err(e) = fs::create_dir_all(dir).await {
            tracing::error!("Error creating {dir} folder. Error: {e}");
        } else {
            tracing::info!("Created {dir} folder");
        }
    }
}
pub async fn define_db_functions() {
    let query = "
        DEFINE FUNCTION fn::loginAdmin($institution:record, $email:string, $pass:string){
            let $admin = select * from admin where institution=$institution and email=$email;
            let $admin = array::first($admin);
            let $teste =
            if $admin=NONE then
                return false;
            else
                return crypto::argon2::compare($admin.password, $pass);
            end;
            return
            if $teste then
                {update admin set reset=NONE where institution=$institution and email=$email;
                return { user: $admin.id, institution: $admin.institution }; }
            end;
        };

        DEFINE FUNCTION fn::loginUser($email:string, $pass:string){
            let $user = select * from user where email=$email;
            let $teste = array::first($user);
            let $pas =
                if $teste = NONE then return false;
                else return crypto::argon2::compare($teste.password, $pass);
            end;
            return
            if $pas then
                { update user set reset=NONE where email=$email;
                  return { user: $teste.id, institution: $teste.institution }; }
            end;
        };
    ";

    if let Err(e) = DB.query(query).await {
        tracing::error!("Failed to define server functions: {e}");
        panic!("Could not define database functions");
    } else {
        tracing::info!("Server functions defined successfully");
    }
}

pub async fn redis_get(rid: &str) -> Result<Option<String>, redis::RedisError> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
        tracing::error!("$REDIS_URL environment variable not set");
        panic!("$REDIS_URL is required");
    });

    let client = Rc::open(redis_url)?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.get(rid).await
}

pub async fn redis_set(rid: &str, st: &str) -> Result<String, redis::RedisError> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
        tracing::error!("$REDIS_URL environment variable not set");
        panic!("$REDIS_URL is required");
    });

    let client = Rc::open(redis_url)?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.set(rid, st).await
}

pub async fn redis_del(rid: &str) -> Result<bool, redis::RedisError> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
        tracing::error!("$REDIS_URL environment variable not set");
        panic!("$REDIS_URL is required");
    });

    let client = Rc::open(redis_url)?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.del(rid).await
}
