mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;
use api::user_api::{
    login_user, register_user
};
use std::env;

use repository::user_repo::UserRepo;

#[launch]
async fn rocket() -> _ {

    env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");

    rocket::build()
        .mount("/", routes![register_user, login_user])
        .manage(UserRepo::init().await)
}
