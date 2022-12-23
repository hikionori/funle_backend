use crate::{
    models::info_model::*,
    repository::{infos_repo::InfosRepo, user_repo::UserRepo}, utils::auth::authorize_token,
};
use rocket::{
    http::Status, serde::json::Json, State
};

// User routes
/// It takes a token and an id, checks if the token is valid, and if it is, it returns the info with the
/// given id
/// 
/// Arguments:
/// 
/// * `db`: &State<InfosRepo> - This is the database connection that we created in the main.rs file.
/// * `user_db`: &State<UserRepo>
/// * `token`: The token of the user who is requesting the info.
/// * `id`: The id of the info you want to get
/// 
/// Returns:
/// 
/// A JSON object containing the info with the given id.
#[get("/user/<token>/get/info?<id>")]
pub async fn get_info_user(db: &State<InfosRepo>, user_db: &State<UserRepo>, token: &str, id: &str) -> Result<Json<InfoModel>, Status> {
    if authorize_token(token.to_string(), user_db).await {
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
/// It takes a database connection and an id, and returns a JSON object of the info with that id, or a
/// 404 if it doesn't exist, or a 500 if something else went wrong
/// 
/// Arguments:
/// 
/// * `db`: &State<InfosRepo> - this is the database connection that we created in the main.rs file.
/// * `id`: The id of the info to get.
/// 
/// Returns:
/// 
/// A JSON object containing the info with the given id.
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

/// It takes a reference to the database and returns a JSON object containing all the infos
/// 
/// Arguments:
/// 
/// * `db`: &State<InfosRepo>
/// 
/// Returns:
/// 
/// A vector of InfoModel
#[get("/admin/get/infos")]
pub async fn get_all_infos(db: &State<InfosRepo>) -> Result<Json<Vec<InfoModel>>, Status> {
    let infos = db.get_all_infos().await;
    match infos {
        Ok(infos) => Ok(Json(infos)),
        Err(_) => Err(Status::InternalServerError)
    }
}

/// It takes a JSON object from the request body, converts it into a Rust struct, and then passes it to
/// the database to be saved
/// 
/// Arguments:
/// 
/// * `db`: &State<InfosRepo> - this is the database connection. It's a reference to the database
/// connection that we created in the main function.
/// * `info`: Json<InfoModel>
/// 
/// Returns:
/// 
/// A Status
#[post("/admin/create/info", data="<info>")]
pub async fn create_info(db: &State<InfosRepo>, info: Json<InfoModel>) -> Status {
    let info = info.into_inner();
    let create_result = db.create_info(info).await;
    match create_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

/// It takes a database connection and an id, and returns a status code
/// 
/// Arguments:
/// 
/// * `db`: &State<InfosRepo> - this is the database connection that we created in the main.rs file.
/// * `id`: &str - The id of the info to delete.
/// 
/// Returns:
/// 
/// A Status
#[delete("/admin/del/info?<id>")]
pub async fn delete_info(db: &State<InfosRepo>, id: &str) -> Status {
    let delete_result = db.delete_info(&id.to_string()).await;
    match delete_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

/// It takes a database connection, an id, and a JSON object, and returns a status code
/// 
/// Arguments:
/// 
/// * `db`: &State<InfosRepo> - this is the database connection that we created in the main.rs file.
/// * `id`: The id of the info to update
/// * `info`: Json<InfoModel> - This is the model that we're going to be updating.
/// 
/// Returns:
/// 
/// A Status
#[put("/admin/update/info?<id>", data="<info>")]
pub async fn update_info(db: &State<InfosRepo>, id: &str, info: Json<InfoModel>) -> Status {
    let info = info.into_inner();
    let update_result = db.update_info(&id.to_string(), info).await;
    match update_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}