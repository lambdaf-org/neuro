use crate::models::validate::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
impl Validate for LoginPayload {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.email.trim().is_empty() {
            errors.push("email is required");
        }
        if self.password.is_empty() {
            errors.push("password is required");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}
impl Validate for RegisterPayload {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.email.trim().is_empty() {
            errors.push("email is required");
        }
        if self.username.trim().is_empty() {
            errors.push("username is required");
        }
        if self.password.is_empty() {
            errors.push("password is required");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Clone)]
pub struct MiddlewareData {
    pub user_id: Uuid,
}

#[derive(Clone, Serialize, ToSchema)]
pub struct LoginRes {
    pub user_id: Uuid,
    pub email: String,
    pub access_token: String,
    pub is_banned: bool,
}
