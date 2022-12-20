#![allow(dead_code, unused_imports)]

use std::any;

use mongodb::results::InsertOneResult;
use rocket::request::FromParam;
use rocket::{
    http::Status,
    serde::json::Json,
    // serde::{Deserialize, Serialize},
    State,
};
use serde::{Deserialize, Serialize};

use crate::utils::auth::authorize_token;
use crate::{
    models::tests_model::{TestModel, TestModelWithActions},
    repository::{
        tests_repo::TestsRepo, tests_with_actions_repo::TestsRepo as TActionRepo,
        user_repo::UserRepo,
    },
};

// * Admin API routes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllTests {
    pub tests: Vec<TestModel>,
    pub tests_with_actions: Vec<TestModelWithActions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    ChoiceTest,
    ActionTest,
}

impl<'r> FromParam<'r> for TestType {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "choice" => Ok(TestType::ChoiceTest),
            "action" => Ok(TestType::ActionTest),
            _ => Err("Invalid test type"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestRes<T, A> {
    ChoiceTest(T),
    ActionTest(A),
}

impl<T, A> TestRes<T, A> {
    // if T == TestModel return TestModel, else return TestModelWithActions
    pub fn get_test(&self) -> &T {
        match self {
            TestRes::ChoiceTest(test) => test,
            TestRes::ActionTest(test) => unimplemented!(),
        }
    }

    pub fn get_test_with_actions(&self) -> &A {
        match self {
            TestRes::ChoiceTest(test) => unimplemented!(),
            TestRes::ActionTest(test) => test,
        }
    }
}

#[get("/admin/get/all")]
pub async fn get_all_tests(
    test_db: &State<TestsRepo>,
    ta_db: &State<TActionRepo>,
) -> Result<Json<AllTests>, Status> {
    let tests = test_db.get_all_tests().await.unwrap();
    let tests_with_actions = ta_db.get_all_tests().await.unwrap();
    let all_tests = AllTests {
        tests,
        tests_with_actions,
    };
    Ok(Json(all_tests))
}

#[get("/admin/get/test?<id>")]
pub async fn get_test_by_id(
    db: &State<TestsRepo>,
    ta_db: &State<TActionRepo>,
    id: &str,
) -> Result<Json<TestRes<TestModel, TestModelWithActions>>, Status> {
    let test = db.get_test_by_id(&id.to_string()).await.unwrap();
    let test_with_actions = ta_db.get_test_by_id(id).await.unwrap();
    match (test, test_with_actions) {
        (Some(test), None) => Ok(Json(TestRes::ChoiceTest(test))),
        (None, Some(test_with_actions)) => Ok(Json(TestRes::ActionTest(test_with_actions))),
        // (Some(test), Some(test_with_actions)) =>
        _ => Err(Status::InternalServerError),
    }
}

#[post("/admin/<test_type>/create/test", data = "<test>")]
pub async fn create_test(
    db: &State<TestsRepo>,
    adb: &State<TActionRepo>,
    test: Json<TestRes<TestModel, TestModelWithActions>>,
    test_type: TestType,
) -> Result<Json<InsertOneResult>, Status> {
    match test_type {
        TestType::ChoiceTest => {
            let test = test.into_inner();
            let result = db.create_test(test.get_test().to_owned()).await;
            match result {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        TestType::ActionTest => {
            let test = test.into_inner();
            let result = adb.create_test(test.get_test_with_actions().to_owned()).await;
            match result {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
    }
}

#[put("/admin/<test_type>/update/test?<id>", data = "<test>")]
pub async fn update_test(
    db: &State<TestsRepo>,
    adb: &State<TActionRepo>,
    id: &str,
    test: Json<TestRes<TestModel, TestModelWithActions>>,
    test_type: TestType
) -> Result<Status, Status> {
    match test_type {
        TestType::ChoiceTest => {
            let test = test.into_inner();
            let result = db.update_test_by_id(&id.to_string(), test.get_test().to_owned()).await;
            match result {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        TestType::ActionTest => {
            let test = test.into_inner();
            let result = adb.update_test(id, test.get_test_with_actions().to_owned()).await;
            match result {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::InternalServerError),
            }
        }
    }
}

#[delete("/admin/delete/test?<id>")]
pub async fn delete_test(db: &State<TestsRepo>, adb: &State<TActionRepo>, id: &str) -> Result<Status, Status> {
    // Delete the test from the test table
    let test_deleted = db.delete_test(&id.to_string()).await;
    // Delete the test from the test action table
    let test_action_deleted = adb.delete_test(id).await;
    // Check the results
    match (test_deleted, test_action_deleted) {
        (Ok(_), Ok(_)) => Ok(Status::Ok),
        _ => Err(Status::InternalServerError),
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
    adb: &State<TActionRepo>,
    user_db: &State<UserRepo>,
    id: &str,
    token: &str,
) -> Result<Json<TestRes<TestModel, TestModelWithActions>>, Status> {
    let access = authorize_token(token.to_string(), user_db).await;
    if access {
        let test = db.get_test_by_id(&id.to_string()).await.unwrap();
        let test_with_actions = adb.get_test_by_id(id).await.unwrap();
        match (test, test_with_actions) {
            (Some(test), None) => Ok(Json(TestRes::ChoiceTest(test))),
            (None, Some(test_with_actions)) => Ok(Json(TestRes::ActionTest(test_with_actions))),
            // (Some(test), Some(test_with_actions)) =>
            _ => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::Unauthorized)
    }
}
