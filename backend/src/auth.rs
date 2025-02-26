use crate::{
    // admin::Admin,
    dbconn::{self, redis_del, redis_get, redis_set, DB},
};
use auth_module::authentication_server::Authentication;
use chrono::Utc;
use core::fmt;
use redis::RedisError;
use reqwest::header::HeaderMap;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use surrealdb::sql::Thing;
use surrealdb::Error;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use self::auth_module::{
    Ack, AdminLoginRequest, LoginResponse, RefreshRequest, RefreshResponse, UserLoginRequest,
    ValidateTokenRequest,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(PartialEq, Debug)]
pub enum Privilege {
    ADMIN,
    USER,
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

#[derive(Debug, Clone, Deserialize)]
pub struct Admin {
    id: Thing,
    email: String,
    pub institution: Thing,
}

#[derive(PartialEq, Debug)]
pub enum Type {
    AUTH,
    REFRESH,
    RESET,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::AUTH => write!(f, "AUTH"),
            Type::REFRESH => write!(f, "REFRESH"),
            Type::RESET => write!(f, "RESET"),
        }
    }
}
impl fmt::Display for Privilege {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Privilege::ADMIN => write!(f, "ADMIN"),
            Privilege::USER => write!(f, "USER"),
        }
    }
}

impl FromStr for Type {
    type Err = ();
    fn from_str(input: &str) -> Result<Type, Self::Err> {
        match input.to_uppercase().as_str() {
            "AUTH" => Ok(Type::AUTH),
            "REFRESH" => Ok(Type::REFRESH),
            "RESET" => Ok(Type::RESET),
            _ => Err(()),
        }
    }
}
impl FromStr for Privilege {
    type Err = ();
    fn from_str(input: &str) -> Result<Privilege, Self::Err> {
        match input.to_uppercase().as_str() {
            "ADMIN" => Ok(Privilege::ADMIN),
            "USER" => Ok(Privilege::USER),
            _ => Err(()),
        }
    }
}

pub mod auth_module {
    use tonic::include_proto;
    include_proto!("auth");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("auth_descriptor");
}

#[derive(Serialize, Deserialize, Clone)]
struct Body {
    institution: Thing,
    user: Thing,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: String,
    pub ins: String,
    pub exp: usize,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshClaims {
    pub rid: String,
    pub uid: String,
    pub exp: usize,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ResetClaims {
    pub rid: String,
    pub uid: String,
    pub exp: usize,
}

const BACKEND_SECRET: &[u8] = b"H7DywKBPbevaHxM6du";

pub fn create_jwt_auth(
    uid: &str,
    ins: &str,
    role: Privilege,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        uid: uid.to_owned(),
        ins: ins.to_string(),
        exp: expiration as usize,
    };
    let mut header = Header::new(Algorithm::HS512);
    header.cty = Some(Type::AUTH.to_string());
    header.typ = Some(role.to_string());
    encode(&header, &claims, &EncodingKey::from_secret(BACKEND_SECRET))
}

pub async fn create_jwt_refresh(
    uid: &str,
    role: Privilege,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30 * 6))
        .expect("valid timestamp")
        .timestamp();

    let rid = Uuid::new_v4().to_string();
    let claims = RefreshClaims {
        rid: rid.clone(),
        uid: uid.to_owned(),
        exp: expiration as usize,
    };
    let mut header = Header::new(Algorithm::HS512);
    header.cty = Some(Type::REFRESH.to_string());
    header.typ = Some(role.to_string());
    let _ = redis_set(&rid, uid).await;
    encode(&header, &claims, &EncodingKey::from_secret(BACKEND_SECRET))
}

pub async fn create_jwt_reset(
    uid: &str,
    role: Privilege,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(10))
        .expect("valid timestamp")
        .timestamp();

    let random_id = Uuid::new_v4().to_string();
    let claims = ResetClaims {
        rid: random_id.clone(),
        uid: uid.to_owned(),
        exp: expiration as usize,
    };
    let mut header = Header::new(Algorithm::HS512);
    header.cty = Some(Type::RESET.to_string());
    header.typ = Some(role.to_string());
    let _ = redis_set(&random_id, uid).await;
    encode(&header, &claims, &EncodingKey::from_secret(BACKEND_SECRET))
}

pub fn decode_jwt(token: &str) -> Result<(Header, Claims), jsonwebtoken::errors::ErrorKind> {
    let decoded: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        token,
        &DecodingKey::from_secret(BACKEND_SECRET),
        &Validation::new(Algorithm::HS512),
    );
    if let Ok(a) = decoded {
        if a.claims.exp >= chrono::Utc::now().timestamp() as usize {
            return Ok((a.header, a.claims));
        }
    }
    Err(jsonwebtoken::errors::ErrorKind::InvalidToken)
}

pub fn decode_refresh(
    token: &str,
) -> Result<(Header, RefreshClaims), jsonwebtoken::errors::ErrorKind> {
    let decoded: Result<jsonwebtoken::TokenData<RefreshClaims>, jsonwebtoken::errors::Error> =
        decode(
            token,
            &DecodingKey::from_secret(BACKEND_SECRET),
            &Validation::new(Algorithm::HS512),
        );
    if let Ok(a) = decoded {
        if a.claims.exp >= chrono::Utc::now().timestamp() as usize {
            return Ok((a.header, a.claims));
        }
    }
    Err(jsonwebtoken::errors::ErrorKind::InvalidToken)
}

pub async fn decode_reset(
    token: &str,
) -> Result<(Header, ResetClaims), jsonwebtoken::errors::ErrorKind> {
    let decoded: Result<jsonwebtoken::TokenData<ResetClaims>, jsonwebtoken::errors::Error> = decode(
        token,
        &DecodingKey::from_secret(BACKEND_SECRET),
        &Validation::new(Algorithm::HS512),
    );
    if let Ok(a) = decoded {
        if a.claims.exp >= chrono::Utc::now().timestamp() as usize {
            if let Ok(Some(id)) = redis_get(&a.claims.rid).await {
                match id == a.claims.uid {
                    true => return Ok((a.header, a.claims)),
                    false => return Err(jsonwebtoken::errors::ErrorKind::InvalidToken),
                }
            }
        }
    }
    Err(jsonwebtoken::errors::ErrorKind::InvalidToken)
}

pub fn validate_jwt_admin(token: &str) -> bool {
    if let Ok((head, _)) = decode_jwt(token) {
        match (
            Type::from_str(head.cty.unwrap().as_str()).unwrap(),
            Privilege::from_str(head.typ.unwrap().as_str()).unwrap(),
        ) {
            (Type::AUTH, Privilege::ADMIN) => return true,
            _ => return false,
        };
    }
    false
}

pub fn is_db_admin(token: &str) -> bool {
    if let Ok((_, body)) = decode_jwt(token) {
        return matches!(Privilege::from_str(body.ins.as_str()), Ok(Privilege::ADMIN));
    }
    false
}

pub fn validate_jwt_user(token: &str) -> bool {
    if let Ok((head, _)) = decode_jwt(token) {
        match (
            Type::from_str(head.cty.unwrap().as_str()),
            Privilege::from_str(head.typ.unwrap().as_str()),
        ) {
            (Ok(Type::AUTH), Ok(Privilege::USER)) => return true,
            _ => return false,
        };
    }
    false
}

fn validate_refresh_admin(token: &str) -> bool {
    if let Ok((head, _)) = decode_refresh(token) {
        match (
            Type::from_str(head.cty.unwrap().as_str()),
            Privilege::from_str(head.typ.unwrap().as_str()),
        ) {
            (Ok(Type::REFRESH), Ok(Privilege::ADMIN)) => return true,
            _ => return false,
        };
    }
    false
}

fn validate_refresh_user(token: &str) -> bool {
    if let Ok((head, _)) = decode_refresh(token) {
        match (
            Type::from_str(head.cty.unwrap().as_str()),
            Privilege::from_str(head.typ.unwrap().as_str()),
        ) {
            (Ok(Type::RESET), Ok(Privilege::USER)) => return true,
            _ => return false,
        };
    }
    false
}

pub async fn validate_jwt_reset(token: &str) -> bool {
    if let Ok((head, _)) = decode_reset(token).await {
        match (
            Type::from_str(head.cty.unwrap().as_str()),
            Privilege::from_str(head.typ.unwrap().as_str()),
        ) {
            (Ok(Type::RESET), Ok(Privilege::USER)) => return true,
            (Ok(Type::RESET), Ok(Privilege::ADMIN)) => return true,
            _ => return false,
        };
    }
    false
}

pub async fn invalidate_jwt_reset((_, c): (Header, ResetClaims)) -> bool {
    if let Ok(true) = redis_del(&c.rid).await {
        return true;
    }
    false
}

#[derive(Default, Clone)]
pub struct AuthManager {}

#[tonic::async_trait]
impl Authentication for AuthManager {
    async fn admin_login(
        &self,
        request: Request<AdminLoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        if let Ok(mut q) = DB
            .query(format!(
                "return fn::loginAdmin(institution:{}, \"{}\", \"{}\");",
                req.institution, req.email, req.password
            ))
            .await
        {
            if let Ok(Some(q)) = q.take::<Option<Body>>(0) {
                let token = create_jwt_auth(
                    &q.user.id.to_string(),
                    &q.institution.id.to_string(),
                    Privilege::ADMIN,
                );
                let refresh = create_jwt_refresh(&q.user.id.to_string(), Privilege::ADMIN).await;
                match (token, refresh) {
                    (Ok(a), Ok(b)) => {
                        return Ok(Response::new(LoginResponse {
                            auth_token: a,
                            refresh_token: b,
                        }))
                    }
                    (_, _) => return Err(Status::internal("couldn't create token")),
                };
            }
            return Err(Status::internal("auth returned NONE"));
        } else {
            return Err(Status::internal(
                "Could not authenticate, probably fields missing ",
            ));
        }
    }

    async fn user_login(
        &self,
        request: Request<UserLoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let mut q = match DB
            .query(format!(
                "return fn::loginUser(\"{}\", \"{}\");",
                req.email, req.password
            ))
            .await
        {
            Ok(a) => a,
            Err(_) => return Err(Status::internal("Could not authenticate, server problem")),
        };
        if let Ok(Some(q)) = q.take::<Option<Body>>(0) {
            let token = create_jwt_auth(
                &q.user.id.to_string(),
                &q.institution.id.to_string(),
                Privilege::USER,
            );
            let refresh = create_jwt_refresh(&q.user.id.to_string(), Privilege::USER).await;
            match (token, refresh) {
                (Ok(a), Ok(b)) => {
                    return Ok(Response::new(LoginResponse {
                        auth_token: a,
                        refresh_token: b,
                    }))
                }
                (_, _) => return Err(Status::internal("couldn't create token")),
            };
        }
        return Err(Status::internal("auth returned NONE"));
    }

    async fn validate_token_user(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<Ack>, Status> {
        match validate_jwt_user(&request.into_inner().auth_token) {
            true => Ok(Response::new(Ack {})),
            false => Err(Status::permission_denied("")),
        }
    }

    async fn validate_token_admin(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<Ack>, Status> {
        match validate_jwt_admin(&request.into_inner().auth_token) {
            true => Ok(Response::new(Ack {})),
            false => Err(Status::permission_denied("")),
        }
    }

    async fn refresh_token_admin(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let refresh = request.into_inner().refresh_token;
        if !validate_refresh_admin(&refresh) {
            return Err(Status::permission_denied(""));
        }
        let (head, decoded) = decode_refresh(&refresh).unwrap();

        let Ok(Some(_)): Result<Option<String>, RedisError> = dbconn::redis_get(&decoded.rid).await
        else {
            return Err(Status::internal("couldn't create auth from refresh"));
        };
        let Ok(mut admin) = DB
            .query(format!("select * from admin:{}", decoded.uid))
            .await
        else {
            return Err(Status::internal("couldn't query db"));
        };
        let Ok(Some(admin)): Result<Option<Admin>, Error> = admin.take(0) else {
            return Err(Status::internal("couldn't query db"));
        };
        match create_jwt_auth(
            &decoded.uid,
            &admin.institution.id.to_string(),
            Privilege::from_str(&head.typ.unwrap()).unwrap(),
        ) {
            Ok(a) => Ok(Response::new(RefreshResponse { auth_token: a })),
            Err(_) => Err(Status::internal("couldn't create auth from refresh")),
        }
    }

    async fn refresh_token_user(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let refresh = request.into_inner().refresh_token;
        if !validate_refresh_user(&refresh) {
            return Err(Status::permission_denied(""));
        }
        let (head, decoded) = decode_refresh(&refresh).unwrap();
        let Some(_): Option<String> = dbconn::redis_get(&decoded.rid).await.unwrap() else {
            return Err(Status::internal("couldn't create auth from refresh"));
        };
        let Ok(mut user) = DB
            .query(format!("select * from user:{}", decoded.uid))
            .await
        else {
            return Err(Status::internal("couldn't query db"));
        };
        let Ok(Some(user)): Result<Option<Aluno>, Error> = user.take(0) else {
            return Err(Status::internal("couldn't query db"));
        };
        match create_jwt_auth(
            &decoded.uid,
            &user.institution.id.to_string(),
            Privilege::from_str(&head.typ.unwrap()).unwrap(),
        ) {
            Ok(a) => Ok(Response::new(RefreshResponse { auth_token: a })),
            Err(_) => Err(Status::internal("couldn't create auth from refresh")),
        }
    }
}
// ) -> Result<reqwest::Response, reqwest::Error> {
//     let c = reqwest::Client::new();
//     let mut headers = HeaderMap::new();
//     headers.insert("Accept", "application/json".parse().unwrap());
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     headers.insert(
//         "X-Postmark-Server-Token",
//         "abf57950-3fd5-468e-8b52-e51d5b485e4d".parse().unwrap(),
//     );
//     let post = c
//         .post("https://api.postmarkapp.com/email".to_string())
//         .json(&json!({
//         "From": "support@portick.app",
//         "To": email.to_string(),
//         "Subject": subject.to_string(),
//         "HtmlBody": body.to_string(),
//         "MessageStream": "outbound"
//         }))
//         .headers(headers)
//         .send()
//         .await;
//     match post {
//         Ok(a) => Ok(a),
//         Err(e) => Err(e),
//     }
// }
