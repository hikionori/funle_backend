mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use api::user_api::*;
use repository::user_repo::UserRepo;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user, get_user_by_name, delete_user_by_id])
        .manage(UserRepo::init().await)
}
