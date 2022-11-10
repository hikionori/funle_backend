use crate::{models::user_model::User, repository::user_repo::UserRepo};
use mongodb::results::InsertOneResult;
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


#[post("/register/users", data = "<user>")]
pub async fn register_user(db: &State<UserRepo>, user: Json<UserRegister>,) -> Result<Json<InsertOneResult>, Status> {
    let user = user.into_inner();
    let user = User {
        id: None,
        username: user.name,
        email: user.email,
        hashed_password: user.password,
        role: user.role.parse().unwrap(),
    };
    let result = db.create_user(user).await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login/users", data = "<user>")]
pub async fn login_user(db: &State<UserRepo>, user: Json<UserLogin>) -> Result<Json<UserLoginResponse>, Status> {
    let login_data = user.into_inner();
    let user = db.get_user_by_email(&login_data.email).await.unwrap().unwrap();
    if (db.hash_password(login_data.password) == user.hashed_password) && (login_data.email == user.email){
        let access_token = create_access_token(user.id.unwrap().to_string(), user.role.clone()).await.unwrap();
        let refresh_token = create_refresh_token(user.id.unwrap().to_string(), user.role.clone()).await.unwrap();
        let response = UserLoginResponse {
            token: access_token,
            refresh_token: refresh_token,
        };
        Ok(Json(response))
    }
    else {
        Err(Status::Unauthorized)
    }
}