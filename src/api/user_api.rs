use crate::{models::user_model::User, repository::user_repo::UserRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/users", data = "<user>")]
pub async fn create_user(
    db: &State<UserRepo>,
    user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = user.into_inner();
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<name>")]
pub async fn get_user_by_name(
    db: &State<UserRepo>,
    name: String,
) -> Result<Json<Option<User>>, Status> {
    let user = db.get_user_by_name(name).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/users/<id>")]
pub async fn delete_user_by_id(
    db: &State<UserRepo>,
    id: String,
) -> Result<Json<Option<User>>, Status> {
    let user = db.delete_user_by_id(id).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
