use thiserror::Error;

// Constants for error message keywords/patterns from supabase-auth
// These make error matching more maintainable and easier to test
// Single-word constants are keywords used in combination with others
const EMAIL_KEYWORD: &str = "email";
const USERNAME_KEYWORD: &str = "username";
const ALREADY_KEYWORD: &str = "already";
const ALREADY_EXISTS_PATTERN: &str = "already exists";
const PASSWORD_KEYWORD: &str = "password";
const RATE_KEYWORD: &str = "rate";
const TOO_MANY_PATTERN: &str = "too many";
const NOT_CONFIRMED_PATTERN: &str = "not confirmed";
const INVALID_KEYWORD: &str = "invalid";
const CREDENTIALS_KEYWORD: &str = "credentials";
const WRONG_KEYWORD: &str = "wrong";

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Email already registered")]
    EmailAlreadyExists,
    
    #[error("Username already taken")]
    UsernameAlreadyTaken,
    
    #[error("Already exists")]
    AlreadyExists,
    
    #[error("Password too weak")]
    WeakPassword,
    
    #[error("Too many attempts")]
    RateLimited,
    
    #[error("Email not confirmed")]
    EmailNotConfirmed,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Authentication failed: {0}")]
    Other(String),
}

impl AuthError {
    /// Classifies a supabase-auth error into an AuthError variant by examining the error message.
    /// This provides a more structured way to handle errors than direct string matching in handlers.
    /// Uses constants for pattern matching to improve maintainability and testability.
    pub fn from_supabase_error(error: &impl std::fmt::Display) -> Self {
        let err_msg = error.to_string();
        let err_lower = err_msg.to_lowercase();
        
        // Pattern match based on error message content using constants
        // Order matters: more specific patterns should come first
        if err_lower.contains(ALREADY_KEYWORD) && err_lower.contains(EMAIL_KEYWORD) {
            AuthError::EmailAlreadyExists
        } else if err_lower.contains(ALREADY_KEYWORD) && err_lower.contains(USERNAME_KEYWORD) {
            AuthError::UsernameAlreadyTaken
        } else if err_lower.contains(ALREADY_EXISTS_PATTERN) {
            AuthError::AlreadyExists
        } else if err_lower.contains(PASSWORD_KEYWORD) {
            AuthError::WeakPassword
        } else if err_lower.contains(RATE_KEYWORD) || err_lower.contains(TOO_MANY_PATTERN) {
            AuthError::RateLimited
        } else if err_lower.contains(NOT_CONFIRMED_PATTERN) {
            AuthError::EmailNotConfirmed
        } else if err_lower.contains(INVALID_KEYWORD) || err_lower.contains(CREDENTIALS_KEYWORD) || err_lower.contains(WRONG_KEYWORD) {
            AuthError::InvalidCredentials
        } else {
            AuthError::Other(err_msg)
        }
    }
}
