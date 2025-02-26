use crate::{auth, dbconn::DB};
use admin_module::{
    admin_administration_server::AdminAdministration, Ack, CreateAdminRequest, DeleteAdminRequest,
    UpdateAdminRequest,
};
use rand::Rng;
use serde::Deserialize;
use surrealdb::{sql::Thing, Error};
use tonic::{Request, Response, Status};
pub mod admin_module {
    use tonic::include_proto;
    include_proto!("admin");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("admin_descriptor");
}
use crate::admin::AdminManager;
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Admin {
    id: Thing,
    email: String,
    pub institution: Thing,
}

#[derive(Default, Clone)]
pub struct AdminManager {}

#[tonic::async_trait]
impl AdminAdministration for AdminManager {
    async fn create_admin(
        &self,
        request: Request<CreateAdminRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("Auth Invalid")),
        };
        let token = auth.to_str().unwrap().to_string();
        if !auth::validate_jwt_admin(&token) {
            return Err(Status::permission_denied("token not valid"));
        }
        if !auth::is_db_admin(&token) {
            return Err(Status::permission_denied("token not valid"));
        }
        let req = request.into_inner();
        let rr = Admin {
            id: Thing {
                tb: "".to_string(),
                id: surrealdb::sql::Id::String("".to_string()),
            },
            email: req.email,
            institution: Thing {
                tb: "institution".to_string(),
                id: surrealdb::sql::Id::String(req.institution),
            },
        };
        let random_string = rand_string();
        // check if user exists
        if !check_email(rr.email.clone()).await {
            return Err(Status::aborted("User already exists"));
        }
        // check if user institution supported
        if !check_institution(rr.institution.id.to_string().clone()).await {
            return Err(Status::aborted("Institution not supported"));
        }
        let qq = DB
            .query(format!(
                "create user set
            email=\"{}\",
            password=crypto::argon2::generate(\"{}\"),
            institution=institution:{};
        ",
                rr.email, random_string, rr.institution.id
            ))
            .await;
        match qq {
            Ok(_) => {}
            Err(_) => return Err(Status::aborted("couldn't create user")),
        };
        // TODO: should send email to user with credentials after this
        Ok(Response::new(Ack {}))
    }

    async fn update_admin(
        &self,
        request: Request<UpdateAdminRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_admin(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("token not valid"));
        }
        let (_, c) = match auth::decode_jwt(auth.to_str().unwrap()) {
            Ok(a) => a,
            Err(_) => return Err(Status::permission_denied("id's don't match")),
        };
        let req = request.into_inner();
        if !c.uid.eq(req.id.clone().unwrap().id.as_str()) {
            return Err(Status::permission_denied("id's don't match"));
        }
        let mut st = String::from("");
        if let Some(a) = req.email {
            st = format!("{} email=\"{}\",", st, a);
        };
        if let Some(a) = req.password {
            st = format!("{} password=crypto::argon2::generate(\"{}\"),", st, a);
        };
        if st.is_empty() {
            return Ok(Response::new(Ack {}));
        }
        let _ = st.pop();
        st.push(';');
        let q = DB
            .query(format!("update admin:{} set {}", req.id.unwrap().id, st))
            .await;
        match q {
            Ok(_) => Ok(Response::new(Ack {})),
            Err(_) => Err(Status::internal("failed to query the db")),
        }
    }

    async fn delete_admin(
        &self,
        request: Request<DeleteAdminRequest>,
    ) -> Result<Response<Ack>, Status> {
        let auth = request.metadata().get("auth");
        let auth = match auth {
            Some(a) => a,
            None => return Err(Status::aborted("no token")),
        };
        if !auth::validate_jwt_admin(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("token not valid"));
        }
        if !auth::is_db_admin(auth.to_str().unwrap()) {
            return Err(Status::permission_denied("token not valid"));
        }
        let req = request.into_inner();

        let query = DB
            .query(format!("delete admin:{}", req.id.unwrap().id))
            .await;

        match query {
            Ok(_) => return Ok(Response::new(Ack {})),
            Err(_) => return Err(Status::not_found("")),
        };
    }
}

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
    match DB
        .query(format!("select email from admin where email=\"{}\"", email))
        .await
    {
        Ok(mut a) => a.take::<Vec<String>>(0).is_ok(),
        Err(_) => false,
    }
}

fn rand_string() -> String {
    (0..10)
        .map(|_| -> String {
            let a: char = rand::thread_rng().gen_range('1'..='z');
            a.to_string()
        })
        .collect()
}
