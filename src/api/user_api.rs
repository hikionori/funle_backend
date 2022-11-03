use crate::{models::user_model::User, repository::user_repo::UserRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, serde::{Serialize, Deserialize}};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegister {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub pwd_hash: String,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub token: String,
}


#[post("/register/users", data = "<user>")]
pub async fn register_user(
    db: &State<UserRepo>,
    user: Json<UserRegister>,
) -> Result<Json<InsertOneResult>, Status> {
    let user = user.into_inner();
    let user = User {
        id: None,
        name: user.name,
        email: user.email,
        password: user.password,
    };
    let result = db.create_user(user).await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login/users", data = "<user>")]
pub async fn login_user(
    db: &State<UserRepo>,
    user: Json<UserLogin>
) -> Result<Json<UserLoginResponse>, Status> {
    todo!()
}