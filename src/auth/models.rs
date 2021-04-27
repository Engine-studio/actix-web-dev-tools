use super::db::{Auth,AuthSecret};
use std::{
    future::Future,
    pin::Pin
};
use actix_web::{dev, web, FromRequest,HttpRequest};
use crate::error::{
    ApiError,
    ErrorType,
    Result,
};

impl FromRequest for Auth {
    type Config = ();
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Auth>>>>;

    fn from_request(
        req: &HttpRequest, 
        _pl: &mut dev::Payload
    ) -> Self::Future {
        let jwt = req
            .headers()
            .get("jwt")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let secret: Option<&web::Data<AuthSecret>> = req.app_data();
        if secret.is_none() {
            return Box::pin(async move { 
                Err(ApiError{
                    code: 500,
                    message: "Auth secret not set".to_string(),
                    error_type: ErrorType::InternalError,
                }) 
            });
        }
        let secret = secret.unwrap().clone();
        Box::pin(async move {
            Auth::validate_jwt(&jwt, &secret.clone())
        })
    }
}

