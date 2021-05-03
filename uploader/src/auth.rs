use diesel::PgConnection;
use std::sync::Arc;
use crate::db::{
    AuthSecret,
    Auth,
};

#[derive(Clone)]
pub struct EngineAuth {
    secret: AuthSecret,
}

impl EngineAuth {
    pub fn new(secret: AuthSecret) -> Self {
        EngineAuth {
            secret,
        }
    }
}

impl actix_web::guard::Guard for EngineAuth {
    fn check(&self, request: &actix_http::RequestHead) -> bool {
        use std::ops::Index;
        let role = request.uri.path();
        let role: Vec<&str> = role
            .split('/')
            .collect();
        let role = role.index(2);
        println!("{}",role);
        let token = match request.headers.get("jwt") {
            Some(t) => t,
            None => return false,
        };
        println!("{:?}",token);
        let user = match token.to_str() {
            Ok(token) => match Auth::validate_jwt(token,&self.secret) {
                Ok(auth) => auth,
                Err(_) => return false,
            },
            Err(_) => return false,
        };
        println!("{:?}",user);
        let d = user.roles.contains(&role.to_string());
        println!("{:?}",d);
        d
    }
}
