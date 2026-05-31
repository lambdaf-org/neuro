use crate::config::middleware::{extract_bearer_token, validate_jwt};

#[test]
fn auth_header_is_used_as_bearer_token() {
    let token = extract_bearer_token(Some("Bearer header-token"), "");

    assert_eq!(token.as_deref(), Some("Bearer header-token"));
}

#[test]
fn auth_header_takes_precedence_over_ws_query_token() {
    let token = extract_bearer_token(Some("Bearer header-token"), "token=query-token");

    assert_eq!(token.as_deref(), Some("Bearer header-token"));
}

#[test]
fn ws_query_token_is_wrapped_for_bearer_validation() {
    let token = extract_bearer_token(None, "foo=bar&token=query-token&x=1");

    assert_eq!(token.as_deref(), Some("Bearer query-token"));
}

#[actix_web::test]
async fn missing_token_is_rejected_before_jwks_lookup() {
    let result = validate_jwt(None).await;

    assert!(result.is_err());
}
