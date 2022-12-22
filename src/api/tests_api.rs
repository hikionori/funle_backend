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

/// `AllTests` is a struct that contains a vector of `TestModel`s and a vector of
/// `TestModelWithActions`s.
/// 
/// Properties:
/// 
/// * `tests`: A list of all tests that have been run.
/// * `tests_with_actions`: This is a vector of TestModelWithActions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllTests {
    pub tests: Vec<TestModel>,
    pub tests_with_actions: Vec<TestModelWithActions>,
}

/// Creating an enum that has two variants: ChoiceTest and ActionTest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    ChoiceTest,
    ActionTest,
}

/// This is a trait that allows us to convert a string into a TestType enum.
impl<'r> FromParam<'r> for TestType {
    type Error = &'r str;

    /// `from_param` is a function that takes a string and returns a `Result` of either a `TestType` or
    /// a `&'r str`
    /// 
    /// Arguments:
    /// 
    /// * `param`: The parameter to be converted.
    /// 
    /// Returns:
    /// 
    /// A Result<Self, Self::Error>
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "choice" => Ok(TestType::ChoiceTest),
            "action" => Ok(TestType::ActionTest),
            _ => Err("Invalid test type"),
        }
    }
}

/// Creating an enum that has two variants: ChoiceTest and ActionTest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestRes<T, A> {
    ChoiceTest(T),
    ActionTest(A),
}

/// This is a trait that allows us to convert a string into a TestType enum.
impl<T, A> TestRes<T, A> {
    // if T == TestModel return TestModel, else return TestModelWithActions
    /// `match` is a pattern matching construct that allows you to match on the variant of an enum
    /// 
    /// Returns:
    /// 
    /// A reference to the test.
    pub fn get_test(&self) -> &T {
        match self {
            TestRes::ChoiceTest(test) => test,
            TestRes::ActionTest(_) => unimplemented!(),
        }
    }

    /// `get_test_with_actions` returns a reference to the `A` type in the `ActionTest` variant of the
    /// `TestRes` enum
    /// 
    /// Returns:
    /// 
    /// A reference to the action test.
    pub fn get_test_with_actions(&self) -> &A {
        match self {
            TestRes::ChoiceTest(_) => unimplemented!(),
            TestRes::ActionTest(test) => test,
        }
    }
}

/// It gets all the tests from the database and returns them as a JSON object
/// 
/// Arguments:
/// 
/// * `test_db`: &State<TestsRepo> - this is the database connection that we created in the main.rs
/// file.
/// * `ta_db`: &State<TActionRepo> - this is the database that contains the tests with actions.
/// 
/// Returns:
/// 
/// A JSON object containing all tests and tests with actions.
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

/// It takes a test id, and returns a test with that id
/// 
/// Arguments:
/// 
/// * `db`: &State<TestsRepo> - this is the database connection.
/// * `ta_db`: &State<TActionRepo> - this is the database connection for the test_action table
/// * `id`: &str - the id of the test to get
/// 
/// Returns:
/// 
/// A test with the given id.
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

/// It takes a test type, a test, and a database connection, and returns a result of either a JSON
/// object or a status code
/// 
/// Arguments:
/// 
/// * `db`: &State<TestsRepo> - this is the database connection that we created in the main.rs file.
/// * `adb`: &State<TActionRepo> - this is the connection to the database for the action test.
/// * `test`: Json<TestRes<TestModel, TestModelWithActions>>
/// * `test_type`: TestType - this is the type of test we're creating.
/// 
/// Returns:
/// 
/// A JSON object with the result of the insert operation.
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

/// It updates a test in the database
/// 
/// Arguments:
/// 
/// * `db`: &State<TestsRepo> - this is the database connection.
/// * `adb`: &State<TActionRepo> - this is the database connection for the action tests.
/// * `id`: &str - the id of the test to update
/// * `test`: Json<TestRes<TestModel, TestModelWithActions>>
/// * `test_type`: TestType - this is an enum that is defined in the models.rs file. It's used to
/// determine which type of test we're dealing with.
/// 
/// Returns:
/// 
/// Status
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

/// Delete a test from the database
/// 
/// Arguments:
/// 
/// * `db`: &State<TestsRepo> - This is the database connection to the tests table
/// * `adb`: &State<TActionRepo> - This is the test action repository.
/// * `id`: The id of the test to delete
/// 
/// Returns:
/// 
/// A status code
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

/// It gets a test by id, and returns it as a JSON object
/// 
/// Arguments:
/// 
/// * `db`: &State<TestsRepo> - this is the database connection to the tests table
/// * `adb`: &State<TActionRepo> - this is the connection to the database
/// * `user_db`: &State<UserRepo> - this is the user database, which is used to check if the user is
/// authorized to access the test.
/// * `id`: &str - the id of the test
/// * `token`: the token of the user who is requesting the test
/// 
/// Returns:
/// 
/// A test with the given id.
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

/* 
TODO: Add a route to get random test by level
Percentage of tests by type: choice - 70%, action - 30%
*/
