use crate::{
    models::info_model::*,
    repository::infos_repo::InfosRepo,
};
use rocket::{
    http::Status, serde::json::Json, State, serde::{Serialize, Deserialize}
};

// CRUD for Infos
// * admin routes
// create, delete, get, update
// * user routes
// get