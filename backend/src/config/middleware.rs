use std::env;

use actix_web::error::ErrorForbidden;
use log::error;
use log::info;
use supabase_jwt::Claims;
use supabase_jwt::JwksCache;
use uuid::Uuid;

pub async fn validate_jwt(token: Option<String>) -> Result<Uuid, Box<dyn std::error::Error>> {
    let Some(token) = token else {
        error!("Authentication failed: Missing Token");
        return Err(ErrorForbidden("Missing token").into());
    };

    // 1. Initialize the JWKS cache with your Supabase URL
    let supabase_url = env::var("SUPABASE_URL").expect("Could not find SUPABASE_URL env");
    let jwks_url = format!("{supabase_url}/auth/v1/.well-known/jwks.json");
    let jwks_cache = JwksCache::new(jwks_url.as_str());

    // 2. Validate the JWT and extract claims
    match Claims::from_bearer_token(token.as_str(), &jwks_cache).await {
        Ok(claims) => {
            info!(
                "Successfully validated token for user: {}",
                claims.user_id()
            );

            let uuid = Uuid::parse_str(claims.user_id()).map_err(|e| {
                error!("Authentication failed: {e}");
                ErrorForbidden("Authentication failed, try logging in again")
            })?;

            Ok(uuid)
        }
        Err(e) => {
            error!("Authentication failed: {e}");
            Err(ErrorForbidden("Authentication failed").into())
        }
    }
}

pub(crate) fn extract_bearer_token(
    authorization_header: Option<&str>,
    query_string: &str,
) -> Option<String> {
    authorization_header.map(str::to_string).or_else(|| {
        query_string
            .split('&')
            .find_map(|part| part.strip_prefix("token="))
            .map(|token| format!("Bearer {token}"))
    })
}
