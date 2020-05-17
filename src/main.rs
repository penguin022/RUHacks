#![feature(proc_macro_hygiene,decl_macro)]
#![feature(try_trait,specialization)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use std::io::Read;
use rocket::response::Body;
use rocket::request::Form;
use std::path::Path;
use std::fs::File;


#[post("/", format="json", data="<input>")]
pub fn auth(input: Form<crypt::UserAuth>) -> crypt::UserAuthResponse{
    unimplemented!("Wire the Database")
}

pub mod crypt;
pub mod auth;

pub fn main(){
    rocket::ignite().mount("/auth",routes![auth])
        .mount("/",rocket_contrib::serve::StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/RUHacks-UI"))).launch();
}