use crate::repository::user_repo::UserRepo;
use crate::utils::auth::authorize_token;
use rocket::{
    http::Status,
    serde::json::Json,
    serde::{Deserialize, Serialize},
    State,
};

#[derive(Deserialize, Clone, Debug)]
pub struct AuthRequest {
    pub token: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AuthResponse {
    pub user_id: String,
    pub is_valid: bool,
}

#[post("/auth", data = "<auth_request>")]
pub async fn auth(
    auth_request: Json<AuthRequest>,
    db: &State<UserRepo>,
) -> Result<Json<AuthResponse>, Status> {
    let auth_request = auth_request.into_inner();
    let (is_valid, user_id) = authorize_token(auth_request.token, db).await;
    let response = AuthResponse { user_id, is_valid };
    Ok(Json(response))
}
