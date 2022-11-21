use rocket::{http::Status, serde::json::Json, State, serde::{Serialize, Deserialize}};
use crate::utils::auth::authorize_token;
use crate::repository::user_repo::UserRepo;


#[derive(Deserialize, Clone, Debug)]
pub struct AuthRequest {
    pub token: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AuthResponse {
    pub is_valid: bool
}

#[post("/auth", data = "<auth_request>")]
pub async fn auth(auth_request: Json<AuthRequest>, db: &State<UserRepo>) -> Result<Json<AuthResponse>, Status> {
    let auth_request = auth_request.into_inner();
    let is_valid = authorize_token(auth_request.token, db).await;
    let response = AuthResponse {
        is_valid
    };
    Ok(Json(response))
}


