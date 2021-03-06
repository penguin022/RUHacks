use std::io::Read;
use rocket::response::{Body, Responder, status};
use std::option::NoneError;
use rocket::{Request, http, Data};
use std::error::Error;
use std::fmt::Display;
use serde::export::Formatter;
use serde::export::fmt::Debug;
use rocket::request::{FromRequest, Outcome};
use rocket::outcome::IntoOutcome;
use rocket::data::{FromData, Transformed, Transform, FromDataSimple};
use serde::{Serialize,Deserialize};

pub struct UserItem{
    pub(crate) hash_addr: String,
    pub(crate) auth_salt: String,
    pub(crate) passwd: String,
    pub(crate) auth_part: String,
    pub(crate) blind_key: String
}

#[derive(FromForm,Debug)]
pub struct UserAuth{
    userid: String,
    password: String
}


#[derive(Serialize,Deserialize,Debug)]
pub enum UserAuthResponse{
    Error{code: u32,msg: String},
    Success{auth_part: String,blinded_key: String},
    CreateUser{auth_part: String,response_identifier: String}
}

impl<E: Error> From<E> for UserAuthResponse{
    fn from(e: E) -> Self {
        Self::Error{code: 4,msg: e.to_string()}
    }
}

impl UserAuthResponse{
    fn from_result(r: Result<UserItem, UserAuthResponse>) -> Self {
        match r{
            Ok(u) => UserAuthResponse::Success {
                auth_part: base64::encode(&u.auth_part),
                blinded_key: base64::encode(&u.blind_key)
            },
            Err(e) => e
        }
    }
}

fn from_code(code: u32) -> rocket::http::Status{
    match code{
        0 => http::Status::from_code(404),
        1 => http::Status::from_code(401),
        2 => http::Status::from_code(403),
        _ => http::Status::from_code(500)
    }.unwrap()
}

impl<'r> Responder<'r> for UserAuthResponse{
    fn respond_to(self, request: &Request<'_>) -> rocket::response::Result<'r> {
        let st = serde_json::to_string(&self).map_err(|s|rocket::http::Status::from_code(500).unwrap())?;
        if let UserAuthResponse::Error {code,msg} = &self{
            status::Custom(from_code(*code),st).respond_to(request)
        }else{
            st.respond_to(request)
        }
    }
}


#[derive(FromForm,Debug)]
pub struct CreateUser{
    response_identifier: String,
    blinded_key: String
}

#[derive(Debug)]
pub enum AuthError{
    BodyEmpty,
    JsonError(serde_json::Error),
    Unknown(String)
}


impl Display for AuthError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <AuthError as Debug>::fmt(self,f)
    }
}

impl Error for AuthError{}


impl From<NoneError> for AuthError{
    fn from(_: NoneError) -> Self {
        Self::BodyEmpty
    }
}

impl From<serde_json::Error> for AuthError{
    fn from(v: serde_json::Error) -> Self {
        Self::JsonError(v)
    }
}

impl From<String> for AuthError{
    fn from(s: String) -> Self {
        Self::Unknown(s.to_string())
    }
}

