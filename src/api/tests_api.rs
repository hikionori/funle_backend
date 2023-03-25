#![allow(dead_code, unused_imports)]

use std::any;

use mongodb::results::InsertOneResult;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::request::FromParam;
use rocket::{
    http::Status,
    serde::json::Json,
    // serde::{Deserialize, Serialize},
    State,
};
use serde::{Deserialize, Serialize};

// use crate::models::tests_model::{TestModelWithActionsResponse, TestModelResponse};
use crate::utils::auth::authorize_token;
use crate::{
    models::tests_model::{TestModel, TestModelWithActions},
    repository::{
        tests_repo::TestsRepo, tests_with_actions_repo::TestsRepo as TActionRepo,
        user_repo::UserRepo,
    },
};

#[options("/<_..>")]
pub fn options() -> Status {
    Status::Ok
}

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

impl AllTests {
    /// `new` is a constructor for the `AllTests` struct.
    ///
    /// Arguments:
    ///
    /// * `tests`: A vector of `TestModel`s.
    /// * `tests_with_actions`: A vector of `TestModelWithActions`s.
    ///
    /// Returns:
    ///
    /// An instance of `AllTests`.
    pub fn new(tests: Vec<TestModel>, tests_with_actions: Vec<TestModelWithActions>) -> Self {
        Self {
            tests,
            tests_with_actions,
        }
    }
}

/// Creating an enum that has two variants: ChoiceTest and ActionTest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    #[serde(rename = "choice")]
    ChoiceTest,
    #[serde(rename = "action")]
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
#[get("/admin/get/tests/all")]
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
    test_type: &str,
    adb: &State<TActionRepo>,
    test: Json<TestRes<TestModel, TestModelWithActions>>,
) -> Result<Json<InsertOneResult>, Status> {
    let test_type = TestType::from_param(test_type).unwrap();
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
            let result = adb
                .create_test(test.get_test_with_actions().to_owned())
                .await;
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
    test_type: &str,
) -> Result<Status, Status> {
    let test_type = TestType::from_param(test_type).unwrap();
    match test_type {
        TestType::ChoiceTest => {
            let test = test.into_inner();
            let result = db
                .update_test_by_id(&id.to_string(), test.get_test().to_owned())
                .await;
            match result {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        TestType::ActionTest => {
            let test = test.into_inner();
            let result = adb
                .update_test(id, test.get_test_with_actions().to_owned())
                .await;
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
#[delete("/admin/<test_type>/delete/test?<id>")]
pub async fn delete_test(
    db: &State<TestsRepo>,
    adb: &State<TActionRepo>,
    id: &str,
    test_type: &str,
) -> Result<Status, Status> {
    
    let test_type = TestType::from_param(test_type).unwrap();
    match test_type {
        TestType::ChoiceTest => {
            let result = db.delete_test(&id.to_string()).await;
            match result {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        TestType::ActionTest => {
            let result = adb.delete_test(id).await;
            match result {
                Ok(_) => Ok(Status::Ok),
                Err(_) => Err(Status::InternalServerError),
            }
        }
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
    if authorize_token(token.to_string(), user_db).await.0 {
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


#[derive(Debug, Serialize, Deserialize)]
pub struct RandReq<'r> {
    pub level: i32,
    pub number_of_tests: i32,
    pub theme: &'r str,
    pub id: &'r str,
}

/// This function returns a random test for a user based on the level of the test
/// 
/// Arguments:
/// 
/// * `db`: &State<TestsRepo> - this is the database connection to the tests table
/// * `adb`: &State<TActionRepo> - this is the action test database
/// * `user_db`: &State<UserRepo> - this is the user database that we created in the previous step.
/// * `level`: i32 - the level of the test
/// * `id`: The user's id
/// * `token`: The token that the user has received when they logged in.
/// * `number_of_tests`: The number of tests that the user wants to get
/// * `theme`: The theme of the test
/// 
/// Returns:
/// 
/// N number of tests based on the level of the test and what the user has not completed yet.
#[get("/user/<token>/get/test/random", data="<req>")]
pub async fn get_random_test_by_level_user(
    db: &State<TestsRepo>,
    adb: &State<TActionRepo>,
    user_db: &State<UserRepo>,
    token: &str,
    req: Json<RandReq<'_>>,
) -> Result<Json<AllTests>, Status> {
    if authorize_token(token.to_string(), user_db).await.0 {
        let req = req.into_inner();
        let level = req.level;
        let number_of_tests = req.number_of_tests;
        let theme = req.theme;
        let id = req.id;
        
        let user = user_db.get_user_by_id(&id.to_string()).await.unwrap().unwrap();
        let user_level_progress = user.progress.tests;
        let mut choice_tests = db
            .get_all_tests()
            .await
            .unwrap()
            .into_iter()
            .filter(|test| test.level == level && !user_level_progress.contains(&test.id.unwrap().to_string()) && test.theme == *theme)
            .collect::<Vec<TestModel>>();
        let mut action_tests = adb
            .get_all_tests()
            .await
            .unwrap()
            .into_iter()
            .filter(|test| test.level == level && !user_level_progress.contains(&test.id.unwrap().to_string()) && test.theme == *theme)
            .collect::<Vec<TestModelWithActions>>();
        // Create all tests with choice test percentage of 70% and action test percentage of 30%
        choice_tests.shuffle(&mut thread_rng());
        action_tests.shuffle(&mut thread_rng());

        // Get the number of tests that we need to return to the user
        let number_of_tests = number_of_tests as usize;
        choice_tests.truncate(number_of_tests);
        action_tests.truncate(number_of_tests);

        choice_tests.truncate((choice_tests.len() as f32 * 0.7) as usize);
        action_tests.truncate((action_tests.len() as f32 * 0.3) as usize);
        
        let all_tests = AllTests::new(choice_tests, action_tests);

        Ok(Json(all_tests))
    } else {
        Err(Status::Unauthorized)
    }
}
