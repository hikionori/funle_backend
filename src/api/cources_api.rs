use crate::{
    models::cource_model::*,
    repository::{
        cource_repo::CourceRepo,
        user_repo::UserRepo,
    },
    utils::auth::authorize_token,
};

use rocket::{
    http::Status,
    serde::json::Json,
    State
};

//? User routes


/// It returns all the courses in the database.
/// 
/// Arguments:
/// 
/// * `db`: &State<CourceRepo> - this is the database connection that we created in the main.rs file.
/// * `udb`: &State<UserRepo>
/// * `token`: The token of the user who is requesting the data.
/// 
/// Returns:
/// 
/// A vector of CourseModel
#[get("/user/<token>/get/cources/all")]
pub async fn get_all_cources_user(db: &State<CourceRepo>, udb: &State<UserRepo>, token: &str) -> Result<Json<Vec<CourseModel>>, Status> {
    if authorize_token(token.to_string(), udb).await {
        let cources = db.get_all().await;
        match cources {
            Some(cources) => Ok(Json(cources)),
            None => Err(Status::NotFound),
        }
    } else {
        Err(Status::Unauthorized)
    }
}

/// It gets a cource by id.
/// 
/// Arguments:
/// 
/// * `db`: &State<CourceRepo> - this is the database connection.
/// * `udb`: &State<UserRepo>
/// * `token`: The token of the user who is requesting the course.
/// * `id`: &str - The id of the course to get
/// 
/// Returns:
/// 
/// A JSON object of the course with the given ID.
#[get("/user/<token>/get/cource?<id>")]
pub async fn get_cource_user(db: &State<CourceRepo>, udb: &State<UserRepo>, token: &str, id: &str) -> Result<Json<CourseModel>, Status> {
    if authorize_token(token.to_string(), udb).await {
        let cource = db.get(id).await;
        match cource {
            Some(cource) => Ok(Json(cource)),
            None => Err(Status::NotFound),
        }
    } else {
        Err(Status::Unauthorized)
    }
}

//? Admin routes

/// It's a `GET` request that takes a `CourceRepo` from the `State` and returns a `Json` of a `Vec` of
/// `CourseModel`s
/// 
/// Arguments:
/// 
/// * `db`: &State<CourceRepo>
/// 
/// Returns:
/// 
/// A vector of CourseModel
#[get("/admin/get/cources/all")]
pub async fn get_all_cources_admin(db: &State<CourceRepo>) -> Result<Json<Vec<CourseModel>>, Status> {
    let cources = db.get_all().await;
    match cources {
        Some(cources) => Ok(Json(cources)),
        None => Err(Status::NotFound),
    }
}

/// It gets a cource by id from the database.
/// 
/// Arguments:
/// 
/// * `db`: &State<CourceRepo> - This is the database connection.
/// * `id`: &str - The id of the course to get.
/// 
/// Returns:
/// 
/// A JSON object of the course with the given ID.
#[get("/admin/get/cource?<id>")]
pub async fn get_cource_admin(db: &State<CourceRepo>, id: &str) -> Result<Json<CourseModel>, Status> {
    let cource = db.get(id).await;
    match cource {
        Some(cource) => Ok(Json(cource)),
        None => Err(Status::NotFound),
    }
}

/// It takes a `CourceRepo` from the `State` and a `CourseModel` from the `Json` and returns a `Result`
/// of `Status` or `Status`
/// 
/// Arguments:
/// 
/// * `db`: &State<CourceRepo> - This is the database connection.
/// * `cource`: Json<CourseModel>
/// 
/// Returns:
/// 
/// A status code of 200
#[post("/admin/add/cource", data = "<cource>")]
pub async fn add_cource_admin(db: &State<CourceRepo>, cource: Json<CourseModel>) -> Result<Status, Status> {
    db.create(cource.into_inner()).await;
    Ok(Status::Ok)
}

/// Update a cource by id
/// Arguments:
/// * `db`: &State<CourceRepo> - This is the database connection.
/// * `id`: &str - The id of the course to update.
/// * `cource`: Json<CourseModel>
/// 
/// Returns: 
/// A status code of 200
#[put("/admin/update/cource?<id>", data = "<cource>")]
pub async fn update_cource_admin(db: &State<CourceRepo>, id: &str, cource: Json<CourseModel>) -> Result<Status, Status> {
    let cource = db.update(id, cource.into_inner()).await;
    match cource {
        Some(_cource) => Ok(Status::Ok),
        None => Err(Status::NotFound),
    }
}

/// Delete a cource by id
/// 
/// Arguments:
/// 
/// * `db`: &State<CourceRepo> - This is the database connection.
/// * `id`: &str - The id of the course to delete.
/// 
/// Returns:
/// 
/// A status code
#[delete("/admin/delete/cource?<id>")]
pub async fn delete_cource_admin(db: &State<CourceRepo>, id: &str) -> Result<Status, Status> {
    db.delete(id).await;
    Ok(Status::Ok)
}