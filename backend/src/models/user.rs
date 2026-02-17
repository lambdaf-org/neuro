use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct MiddlewareData {
    pub user_id: Uuid,
}

#[derive(Clone, Serialize)]
pub struct LoginRes {
    pub user_id: Uuid,
    pub email: String,
    pub access_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}
