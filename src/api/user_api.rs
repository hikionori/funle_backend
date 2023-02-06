#![allow(dead_code)]

use crate::{
    models::user_model::{UserModel, UserProgress},
    repository::user_repo::UserRepo,
    utils::auth::authorize_token,
};
use mongodb::results::InsertOneResult;
use rocket::{
    http::Status,
    serde::json::Json,
    serde::{Deserialize, Serialize},
    State,
};

use crate::utils::auth::{create_access_token, create_refresh_token};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegister {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub token: String,
    pub refresh_token: String,
}

// * User api routes
// * Auth routes
/// It takes a `UserRegister` struct, converts it to a `UserModel` struct, and then inserts it into the
/// database
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is the database connection that we created in the main.rs file.
/// * `user`: Json<UserRegister>
///
/// Returns:
///
/// A JSON object with the user's ID, username, email, hashed password, role, and progress.
#[post("/user/register/users", data = "<user>")]
pub async fn register_user(
    db: &State<UserRepo>,
    user: Json<UserRegister>,
) -> Result<Json<InsertOneResult>, Status> {
    let user = user.into_inner();
    let user = UserModel {
        id: None,
        username: user.name,
        email: user.email,
        hashed_password: user.password,
        role: user.role.parse().unwrap(),
        progress: UserProgress {
            courses: vec![],
            tests: vec![],
            infos: vec![],
        },
    };
    let result = db.create_user(user).await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// It takes a user's email and password, checks if the password is correct, and if it is, returns a JWT
/// access token and refresh token
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is the database connection that we created in the main.rs file.
/// * `user`: Json<UserLogin> - This is the request body that will be sent to the endpoint.
///
/// Returns:
///
/// A JSON object with the access token and refresh token.
#[post("/user/login/users", data = "<user>")]
pub async fn login_user(
    db: &State<UserRepo>,
    user: Json<UserLogin>,
) -> Result<Json<UserLoginResponse>, Status> {
    let login_data = user.into_inner();
    let user = db
        .get_user_by_email(&login_data.email)
        .await
        .unwrap()
        .unwrap();
    if (db.hash_password(login_data.password) == user.hashed_password)
        && (login_data.email == user.email)
    {
        let access_token = create_access_token(user.id.unwrap().to_string(), user.role)
            .await
            .unwrap();
        let refresh_token = create_refresh_token(user.id.unwrap().to_string(), user.role)
            .await
            .unwrap();
        let response = UserLoginResponse {
            token: access_token,
            refresh_token,
        };
        Ok(Json(response))
    } else {
        Err(Status::Unauthorized)
    }
}

// Routes for work with cources
// * Join to course
/// `JoiningData` is a struct that contains two fields, `course_id` and `user_id`, both of which are
/// strings.
///
/// The `#[derive(Deserialize)]` line is a Rust annotation that tells the compiler to automatically
/// generate code that will deserialize JSON into this type.
///
/// The `pub` keyword means that this type is public, which means that it can be used outside of this
/// module.
///
/// The `String` type is a string type that is provided by the Rust standard library.
///
/// The `JoiningData
///
/// Properties:
///
/// * `course_id`: The ID of the course that the user is joining.
/// * `user_id`: The user's ID.
#[derive(Deserialize)]
pub struct JoiningData {
    pub course_id: String,
    pub user_id: String,
}

/// It takes a token and a joining data struct, checks if the token is valid, and if it is, adds the
/// course to the user
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is the database connection.
/// * `token`: The token of the user who is trying to join the course.
/// * `joining_data`: This is the data that will be sent to the server.
///
/// Returns:
///
/// A status code
#[put("/user/<token>/join/course", data = "<joining_data>")]
pub async fn join_course(
    db: &State<UserRepo>,
    token: String,
    joining_data: Json<JoiningData>,
) -> Result<Status, Status> {
    if authorize_token(token, db).await.0 {
        let joining_data = joining_data.into_inner();
        db.add_cource_to_user(joining_data.user_id, joining_data.course_id)
            .await;
        Ok(Status::Ok)
    } else {
        Err(Status::Unauthorized)
    }
}

// * Leave course
/// It takes a token and a joining data object, and if the token is valid, it removes the course from
/// the user's list of courses
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection.
/// * `token`: The token of the user who is trying to leave the course.
/// * `joining_data`: This is the data that will be sent to the server.
///
/// Returns:
///
/// A status code
#[put("/user/<token>/leave/course", data = "<joining_data>")]
pub async fn leave_course(
    db: &State<UserRepo>,
    token: String,
    joining_data: Json<JoiningData>,
) -> Result<Status, Status> {
    if authorize_token(token, db).await.0 {
        let joining_data = joining_data.into_inner();
        db.remove_cource_from_user(joining_data.user_id, joining_data.course_id)
            .await;
        Ok(Status::Ok)
    } else {
        Err(Status::Unauthorized)
    }
}

// Routes for work with tests
// * Mark test as passed
/// `TestPassingData` is a struct that contains two fields, `test_id` and `user_id`, both of which are
/// strings.
///
/// The `#[derive(Deserialize)]` line is a Rust annotation that tells the Rust compiler to automatically
/// generate code that will deserialize JSON into this type.
///
/// The `pub` keyword means that this type is public, which means that it can be used outside of this
/// module.
///
/// The `String` type is a string type that is provided by the Rust standard library.
///
/// The `test
///
/// Properties:
///
/// * `test_id`: The ID of the test that the user is taking.
/// * `user_id`: The user's ID.
#[derive(Deserialize)]
pub struct TestPassingData {
    pub test_id: String,
    pub user_id: String,
}

/// It takes a token and a test passing data, checks if the token is valid, and if it is, adds the test
/// to the user's list of passed tests
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is a reference to the database connection pool.
/// * `token`: String - the token of the user who is trying to pass the test
/// * `test_passing_data`: Json<TestPassingData> - this is a struct that contains the user_id and
/// test_id.
///
/// Returns:
///
/// Status
#[put("/user/<token>/pass/test", data = "<test_passing_data>")]
pub async fn pass_test(
    db: &State<UserRepo>,
    token: String,
    test_passing_data: Json<TestPassingData>,
) -> Result<Status, Status> {
    if authorize_token(token, db).await.0 {
        let test_passing_data = test_passing_data.into_inner();
        db.add_test_to_user(test_passing_data.user_id, test_passing_data.test_id)
            .await;
        Ok(Status::Ok)
    } else {
        Err(Status::Unauthorized)
    }
}

// Routes for work with user info
// * Add info to user progess
/// It's a struct that has two fields, one called `info_id` and one called `user_id`, both of which are
/// strings.
///
/// The `#[derive(Deserialize)]` line is a Rust annotation that tells the Rust compiler to automatically
/// generate code that will deserialize JSON into this type.
///
/// The `pub` keyword means that this type is public, which means that it can be used by other code in
/// this crate.
///
/// The `struct` keyword means that this is a struct, which is a type that has fields.
///
/// Properties:
///
/// * `info_id`: The ID of the info that the user is passing.
/// * `user_id`: The user's ID.
#[derive(Deserialize)]
pub struct InfoPassingData {
    pub info_id: String,
    pub user_id: String,
}

/// It takes a token and a JSON object containing a user ID and an info ID, and if the token is valid,
/// it adds the info to the user's info list
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is the database connection.
/// * `token`: The token of the user who is passing the info.
/// * `info_passing_data`: This is the data that is passed to the server. It is a JSON object that
/// contains the user_id and info_id.
///
/// Returns:
///
/// A status code.
#[put("/user/<token>/pass/info", data = "<info_passing_data>")]
pub async fn pass_info(
    db: &State<UserRepo>,
    token: String,
    info_passing_data: Json<InfoPassingData>,
) -> Result<Status, Status> {
    if  authorize_token(token, db).await.0 {
        let info_passing_data = info_passing_data.into_inner();
        db.add_info_to_user(info_passing_data.user_id, info_passing_data.info_id)
            .await;
        Ok(Status::Ok)
    } else {
        Err(Status::Unauthorized)
    }
}

// * Admin api routes

/// It deletes a user from the database.
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection that we created in the main.rs file.
/// * `id`: &str - The id of the user to delete.
///
/// Returns:
///
/// A JSON object of the deleted user.
#[delete("/admins/del/user?<id>")]
pub async fn delete_user(
    db: &State<UserRepo>,
    id: &str,
) -> Result<Json<Option<UserModel>>, Status> {
    let result = db.delete_user_by_id(&id.to_string()).await.unwrap();
    Ok(Json(Some(result)))
}

/// `get_user` is a function that takes a `UserRepo` and a `&str` and returns a
/// `Result<Json<Option<UserModel>>, Status>`
///
/// The `UserRepo` is a struct that contains a `Pool` of `SqliteConnection`s. The `&str` is the id of
/// the user we want to get. The `Result` is a `Json` object that contains an `Option` of a `UserModel`
/// or a `Status`
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection that we created in the main.rs file.
/// * `id`: &str - The id of the user we want to get.
///
/// Returns:
///
/// A JSON object of the user with the given id.
#[get("/admins/get/user?<id>")]
pub async fn get_user(db: &State<UserRepo>, id: &str) -> Result<Json<Option<UserModel>>, Status> {
    let result = db.get_user_by_id(&id.to_string()).await.unwrap();
    match result {
        Some(user) => Ok(Json(Some(user))),
        None => Err(Status::NotFound),
    }
}

/// This function is a route handler that takes a database connection as a parameter and returns a JSON
/// array of all users in the database
///
/// Arguments:
///
/// * `db`: &State<UserRepo>
///
/// Returns:
///
/// A vector of UserModel structs
#[get("/admins/get/users")]
pub async fn get_users(db: &State<UserRepo>) -> Result<Json<Vec<UserModel>>, Status> {
    let result = db.get_all_users().await.unwrap();
    Ok(Json(result))
}

/// `update_user` takes a `UserRepo` and a `UserModel` and returns a `Result<Json<Option<UserModel>>,
/// Status>`
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection that we created in the main.rs file.
/// * `id`: &str - The id of the user to update
/// * `user`: Json<UserModel>
///
/// Returns:
///
/// A JSON object of the updated user.
#[put("/admins/update/user?<id>", data = "<user>")]
pub async fn update_user(
    db: &State<UserRepo>,
    id: &str,
    user: Json<UserModel>,
) -> Result<Json<Option<UserModel>>, Status> {
    let result = db
        .put_user_by_id(&id.to_string(), user.into_inner())
        .await
        .unwrap();
    match result {
        Some(user) => Ok(Json(Some(user))),
        None => Err(Status::NotFound),
    }
}

/// `update_user_progress` takes a `UserProgress` struct, and updates the user's progress in the
/// database
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection that we created in the main.rs file.
/// * `id`: The id of the user to update
/// * `progress`: Json<UserProgress>
///
/// Returns:
///
/// A status code
#[put("/admins/update/user/progress?<id>", data = "<progress>")]
pub async fn update_user_progress(
    db: &State<UserRepo>,
    id: &str,
    progress: Json<UserProgress>,
) -> Result<Status, Status> {
    let result = db
        .update_all_progress(id.to_string(), progress.into_inner())
        .await
        .unwrap();
    match result {
        Some(_) => Ok(Status::Ok),
        None => Err(Status::NotFound),
    }
}

// Admins routes for work with courses
// * Add course to user
/// It takes a JSON object with a user_id and a course_id, and adds the course to the user's list of
/// courses
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection.
/// * `joining_data`: This is the data that we are going to send to the server.
///
/// Returns:
///
/// A status code
#[put("/admins/add/course/user", data = "<joining_data>")]
pub async fn add_course_to_user(
    db: &State<UserRepo>,
    joining_data: Json<JoiningData>,
) -> Result<Status, Status> {
    let joining_data = joining_data.into_inner();
    db.add_cource_to_user(joining_data.user_id, joining_data.course_id)
        .await;
    Ok(Status::Ok)
}

// * Remove course from user
/// It removes a course from a user
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - This is the database connection.
/// * `joining_data`: This is the data that we're going to be sending to the server.
///
/// Returns:
///
/// Status::Ok
#[put("/admins/remove/course/user", data = "<joining_data>")]
pub async fn remove_course_from_user(
    db: &State<UserRepo>,
    joining_data: Json<JoiningData>,
) -> Result<Status, Status> {
    let joining_data = joining_data.into_inner();
    db.remove_cource_from_user(joining_data.user_id, joining_data.course_id)
        .await;
    Ok(Status::Ok)
}
// Admins routes for work with tests
// * Add test to user
/// It adds a test to a user
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is a reference to the database.
/// * `test_passing_data`: Json<TestPassingData> - this is the data that will be sent to the server.
///
/// Returns:
///
/// Status::Ok
#[put("/admins/add/test/user", data = "<test_passing_data>")]
pub async fn add_test_to_user(
    db: &State<UserRepo>,
    test_passing_data: Json<TestPassingData>,
) -> Result<Status, Status> {
    let test_passing_data = test_passing_data.into_inner();
    db.add_test_to_user(test_passing_data.user_id, test_passing_data.test_id)
        .await;
    Ok(Status::Ok)
}

// * Remove test from user
/// It removes a test from a user
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is a reference to the database.
/// * `test_passing_data`: Json<TestPassingData> - this is the data that will be sent to the server.
///
/// Returns:
///
/// Status::Ok
#[put("/admins/remove/test/user", data = "<test_passing_data>")]
pub async fn remove_test_from_user(
    db: &State<UserRepo>,
    test_passing_data: Json<TestPassingData>,
) -> Result<Status, Status> {
    let test_passing_data = test_passing_data.into_inner();
    db.remove_test_from_user(test_passing_data.user_id, test_passing_data.test_id)
        .await;
    Ok(Status::Ok)
}
// Admins routes for work with infos
// * Add info to user
/// It adds an info to a user
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is the database connection that we're going to use to access the
/// database.
/// * `info_passing_data`: This is the data that is passed to the function. It is a JSON object that
/// contains the user_id and info_id.
///
/// Returns:
///
/// A status code.
#[put("/admins/add/info/user", data = "<info_passing_data>")]
pub async fn add_info_to_user(
    db: &State<UserRepo>,
    info_passing_data: Json<InfoPassingData>,
) -> Result<Status, Status> {
    let info_passing_data = info_passing_data.into_inner();
    db.add_info_to_user(info_passing_data.user_id, info_passing_data.info_id)
        .await;
    Ok(Status::Ok)
}

// * Remove info from user
/// It removes an info from a user
///
/// Arguments:
///
/// * `db`: &State<UserRepo> - this is the database connection that we're using.
/// * `info_passing_data`: This is the data that is passed to the server. It is a JSON object that
/// contains the user_id and info_id.
///
/// Returns:
///
/// A status code.
#[put("/admins/remove/info/user", data = "<info_passing_data>")]
pub async fn remove_info_from_user(
    db: &State<UserRepo>,
    info_passing_data: Json<InfoPassingData>,
) -> Result<Status, Status> {
    let info_passing_data = info_passing_data.into_inner();
    db.remove_info_from_user(info_passing_data.user_id, info_passing_data.info_id)
        .await;
    Ok(Status::Ok)
}
