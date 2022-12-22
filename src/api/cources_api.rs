use crate::{
    models::cource_model::*,
    repository::{
        cource_repo::CourceRepo,
        infos_repo::InfosRepo,
        user_repo::UserRepo,
        tests_repo::TestsRepo,
        tests_with_actions_repo::TestsRepo as ATestsRepo,
    },
    utils::auth::authorize_token,
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

// User routes
// get all cources

// Admin routes
// get all cources
// get cource by id
// redact cource
// delete cource
// add cource

