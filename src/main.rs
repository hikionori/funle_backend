mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;
use api::{
    user_api::*,
    tests_api::*
};
use std::env;

use repository::{
    user_repo::UserRepo,
    tests_repo::TestsRepo,
    tests_with_actions_repo::TestsRepo as TActionRepo // TestActionRepo -> TActionRepo
};

#[launch]
async fn rocket() -> _ {

    env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");

    rocket::build()
        .mount("/", routes![register_user, login_user, get_test_by_id_user])
        .mount("/", routes![get_test_by_id]) // user
        .mount("/", routes![get_all_tests, delete_test]) // admin
        .manage(UserRepo::init().await)
        .manage(TestsRepo::init().await)
        .manage(TActionRepo::init().await)
}
