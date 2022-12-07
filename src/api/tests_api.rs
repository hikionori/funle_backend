#![allow(dead_code, unused_imports)]

use mongodb::results::InsertOneResult;
use rocket::{
    http::Status,
    serde::json::Json,
    // serde::{Deserialize, Serialize},
    State,
};

use crate::utils::auth::authorize_token;
use crate::{
    models::tests_model::TestModel,
    repository::{tests_repo::TestsRepo, user_repo::UserRepo},
};

// * Admin API routes

#[get("/admin/get/all")]
pub async fn get_all_tests(db: &State<TestsRepo>) -> Result<Json<Vec<TestModel>>, Status> {
    let result = db.get_all_tests().await.unwrap();
    if result.is_empty() {
        Err(Status::NoContent)
    } else {
        Ok(Json(result))
    }
}

#[get("/admin/get/test?<id>")]
pub async fn get_test_by_id(db: &State<TestsRepo>, id: &str) -> Result<Json<TestModel>, Status> {
    let test = db.get_test_by_id(&id.to_string()).await.unwrap();
    match test {
        Some(test) => Ok(Json(test)),
        None => Err(Status::NotFound),
    }
}

#[post("/admin/create/test", data = "<test>")]
pub async fn create_test(
    db: &State<TestsRepo>,
    test: Json<TestModel>,
) -> Result<Json<InsertOneResult>, Status> {
    let test = test.into_inner();
    let result = db.create_test(test).await;
    match result {
        Ok(test) => Ok(Json(test)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/admin/update/test?<id>", data = "<test>")]
pub async fn update_test(
    db: &State<TestsRepo>,
    id: &str,
    test: Json<TestModel>,
) -> Result<Status, Status> {
    let test = test.into_inner();
    let result = db.update_test_by_id(&id.to_string(), test).await;
    match result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/admin/delete/test?<id>")]
pub async fn delete_test(db: &State<TestsRepo>, id: &str) -> Result<Status, Status> {
    let result = db.delete_test(&id.to_string()).await;
    match result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

/* * User API routes
* For user routes we need to check if user is authorized
* and if he has access to this test
* Example json for get test by id:
* {
*  token: <token>,
*  test_id: <test_id>
* }
*/

#[get("/user/<token>/get/test?<id>")]
pub async fn get_test_by_id_user(
    db: &State<TestsRepo>,
    user_db: &State<UserRepo>,
    id: &str,
    token: &str,
) -> Result<Json<TestModel>, Status> {
    let access = authorize_token(token.to_string(), user_db).await;
    if access {
        let test = db.get_test_by_id(&id.to_string()).await.unwrap();
        match test {
            Some(test) => Ok(Json(test)),
            None => Err(Status::NotFound),
        }
    }
    else {
        Err(Status::Unauthorized)
    }
}
