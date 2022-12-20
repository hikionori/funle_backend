use crate::{
    models::info_model::*,
    repository::{infos_repo::InfosRepo, user_repo::UserRepo}, utils::auth::authorize_token,
};
use rocket::{
    http::Status, serde::json::Json, State, serde::{Serialize, Deserialize}, fairing::Info
};

// User routes
#[get("/user/<token>/get/info?<id>")]
pub async fn get_info_user(db: &State<InfosRepo>, user_db: &State<UserRepo>, token: String, id: &str) -> Result<Json<InfoModel>, Status> {
    if authorize_token(token, user_db).await {
        let info = db.get_info(&id.to_string()).await;
        match info {
            Ok(info) => {
                match info {
                    Some(info) => Ok(Json(info)),
                    None => Err(Status::NotFound)
                }
            },
            Err(_) => {
                Err(Status::InternalServerError)
            }
        }
    } else {
        Err(Status::Unauthorized)
    }
}

// Admin routes
#[get("/admin/get/info?<id>")]
pub async fn get_info_admin(db: &State<InfosRepo>, id: &str) -> Result<Json<InfoModel>, Status> {
    let info = db.get_info(&id.to_string()).await;
    match info {
        Ok(info) => {
            match info {
                Some(info) => Ok(Json(info)),
                None => Err(Status::NotFound)
            }
        },
        Err(_) => {
            Err(Status::InternalServerError)
        }
    }
}

#[get("/admin/get/infos")]
pub async fn get_all_infos(db: &State<InfosRepo>) -> Result<Json<Vec<InfoModel>>, Status> {
    let infos = db.get_all_infos().await;
    match infos {
        Ok(infos) => Ok(Json(infos)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[post("/admin/create/info", data="<info>")]
pub async fn create_info(db: &State<InfosRepo>, info: Json<InfoModel>) -> Status {
    let info = info.into_inner();
    let create_result = db.create_info(info).await;
    match create_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

#[delete("/admin/del/info?<id>")]
pub async fn delete_info(db: &State<InfosRepo>, id: &str) -> Status {
    let delete_result = db.delete_info(&id.to_string()).await;
    match delete_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

#[put("/admin/update/info?<id>", data="<info>")]
pub async fn update_info(db: &State<InfosRepo>, id: &str, info: Json<InfoModel>) -> Status {
    let info = info.into_inner();
    let update_result = db.update_info(&id.to_string(), info).await;
    match update_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}