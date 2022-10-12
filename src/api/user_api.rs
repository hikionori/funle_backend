use crate::{
    models::user_model::User,
    repository::user_repo::UserRepo,
};
use mongodb::results::InsertOneResult;
use rocket::{
    http::Status,
    serde::json::Json,
    State,
};

#[post("/users", data = "<user>")]
pub async fn create_user(db: &State<UserRepo>, user: Json<User>) -> Result<Json<InsertOneResult>, Status> {
    let data = user.into_inner();
    let user_detail = db.create(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}