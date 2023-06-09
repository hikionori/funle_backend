mod api;
mod models;
mod repository;
mod utils;

#[macro_use]
extern crate rocket;
use api::{
    auth_api::auth,
    cources_api::{
        add_cource_admin, delete_cource_admin, get_all_cources_admin, get_all_cources_user,
        get_cource_admin, get_cource_user, update_cource_admin,
    },
    infos_api::{
        create_info, delete_info, get_all_infos, get_info_admin, get_info_user, update_info,
    },
    tests_api::{
        create_test, delete_test, get_all_tests, get_random_test_by_level_user, get_test_by_id,
        get_test_by_id_user, options, update_test,
    },
    user_api::{
        add_course_to_user, add_info_to_user, add_test_to_user, delete_user, get_user,
        get_user_info, get_users, join_course, leave_course, login_user, pass_info, pass_node,
        pass_test, register_user, remove_course_from_user, remove_info_from_user,
        remove_test_from_user, update_user, update_user_progress,
    },
};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use std::env;

use repository::{
    cource_repo::CourceRepo,
    infos_repo::InfosRepo,
    tests_repo::TestsRepo,
    tests_with_actions_repo::TestsRepo as TActionRepo, // TestActionRepo -> TActionRepo
    user_repo::UserRepo,
};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    #[cfg(debug_assertions)]
    env::set_var("MONGO_URL", "mongodb://root:root@localhost:27017/");

    rocket::build()
        .attach(CORS)
        // Options
        .mount("/api", routes![options])
        // User API
        .mount(
            "/api",
            routes![
                register_user,
                login_user,
                get_user_info,
                join_course,
                leave_course,
                pass_info,
                pass_test,
                pass_node,
                get_test_by_id_user,
                get_random_test_by_level_user,
                get_all_cources_user,
                get_cource_user,
                get_info_user
            ],
        ) // user
        .mount(
            "/api",
            routes![
                get_users,
                get_user,
                delete_user,
                update_user,
                add_course_to_user,
                remove_course_from_user,
                add_info_to_user,
                remove_info_from_user,
                add_test_to_user,
                remove_test_from_user,
                update_user_progress,
                get_all_tests,
                get_test_by_id,
                create_test,
                update_test,
                delete_test,
                get_all_cources_admin,
                get_cource_admin,
                add_cource_admin,
                update_cource_admin,
                delete_cource_admin,
                get_info_admin,
                get_all_infos,
                create_info,
                update_info,
                delete_info
            ],
        ) // admin
        // Auth API
        .mount("/api", routes![auth])
        .manage(UserRepo::init().await)
        .manage(TestsRepo::init().await)
        .manage(TActionRepo::init().await)
        .manage(InfosRepo::init().await)
        .manage(CourceRepo::init().await)
}
