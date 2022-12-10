#![allow(dead_code)]

use crate::{models::user_model::{UserModel, UserProgress}, repository::user_repo::UserRepo};
use mongodb::results::{InsertOneResult, DeleteResult};
use rocket::{http::Status, serde::json::Json, State, serde::{Serialize, Deserialize}};

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
#[post("/user/register/users", data = "<user>")]
pub async fn register_user(db: &State<UserRepo>, user: Json<UserRegister>,) -> Result<Json<InsertOneResult>, Status> {
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

#[post("/user/login/users", data = "<user>")]
pub async fn login_user(db: &State<UserRepo>, user: Json<UserLogin>) -> Result<Json<UserLoginResponse>, Status> {
    let login_data = user.into_inner();
    let user = db.get_user_by_email(&login_data.email).await.unwrap().unwrap();
    if (db.hash_password(login_data.password) == user.hashed_password) && (login_data.email == user.email){
        let access_token = create_access_token(user.id.unwrap().to_string(), user.role).await.unwrap();
        let refresh_token = create_refresh_token(user.id.unwrap().to_string(), user.role).await.unwrap();
        let response = UserLoginResponse {
            token: access_token,
            refresh_token,
        };
        Ok(Json(response))
    }
    else {
        Err(Status::Unauthorized)
    }
}

// * Admin api routes

#[post("/admins/del/user?<id>")]
pub async fn delete_user(db: &State<UserRepo>, id: &str) -> Result<Json<Option<UserModel>>, Status> {
    let result = db.delete_user_by_id(&id.to_string()).await.unwrap();
    Ok(Json(result))
}

#[get("/admins/get/user?<id>")]
pub async fn get_user(db: &State<UserRepo>, id: &str) -> Result<Json<Option<UserModel>>, Status> {
    let result = db.get_user_by_id(&id.to_string()).await.unwrap();
    match result {
        Some(user) => Ok(Json(Some(user))),
        None => Err(Status::NotFound),
    }
}

#[get("/admins/get/users")]
pub async fn get_users(db: &State<UserRepo>) -> Result<Json<Vec<UserModel>>, Status> {
    let result = db.get_all_users().await.unwrap();
    Ok(Json(result))
}

#[post("/admins/update/user?<id>", data = "<user>")]
pub async fn update_user(db: &State<UserRepo>, id: &str, user: Json<UserModel>) -> Result<Json<Option<UserModel>>, Status> {
    let result = db.put_user_by_id(&id.to_string(), user.into_inner()).await.unwrap();
    match result {
        Some(user) => Ok(Json(Some(user))),
        None => Err(Status::NotFound),
    }
}

#[post("/admins/update/user/progress?<id>", data = "<progress>")]
pub async fn update_user_progress(db: &State<UserRepo>, id: &str, progress: Json<UserProgress>) -> Result<Status, Status> {
    let result = db.update_all_progress(id.to_string(), progress.into_inner()).await.unwrap();
    match result {
        Some(_) => Ok(Status::Ok),
        None => Err(Status::NotFound),
    }
}