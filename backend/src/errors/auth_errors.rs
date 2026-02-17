use thiserror::Error;

// Constants for error message patterns from supabase-auth
// These make error matching more maintainable and easier to test
const EMAIL_ALREADY_EXISTS_PATTERN: &str = "email";
const USERNAME_ALREADY_TAKEN_PATTERN: &str = "username";
const ALREADY_EXISTS_PATTERN: &str = "already exists";
const ALREADY_PATTERN: &str = "already";
const PASSWORD_PATTERN: &str = "password";
const RATE_LIMITED_PATTERN: &str = "rate";
const TOO_MANY_PATTERN: &str = "too many";
const NOT_CONFIRMED_PATTERN: &str = "not confirmed";
const INVALID_PATTERN: &str = "invalid";
const CREDENTIALS_PATTERN: &str = "credentials";
const WRONG_PATTERN: &str = "wrong";

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
        if err_lower.contains(ALREADY_PATTERN) && err_lower.contains(EMAIL_ALREADY_EXISTS_PATTERN) {
            AuthError::EmailAlreadyExists
        } else if err_lower.contains(ALREADY_PATTERN) && err_lower.contains(USERNAME_ALREADY_TAKEN_PATTERN) {
            AuthError::UsernameAlreadyTaken
        } else if err_lower.contains(ALREADY_EXISTS_PATTERN) {
            AuthError::AlreadyExists
        } else if err_lower.contains(PASSWORD_PATTERN) {
            AuthError::WeakPassword
        } else if err_lower.contains(RATE_LIMITED_PATTERN) || err_lower.contains(TOO_MANY_PATTERN) {
            AuthError::RateLimited
        } else if err_lower.contains(NOT_CONFIRMED_PATTERN) {
            AuthError::EmailNotConfirmed
        } else if err_lower.contains(INVALID_PATTERN) || err_lower.contains(CREDENTIALS_PATTERN) || err_lower.contains(WRONG_PATTERN) {
            AuthError::InvalidCredentials
        } else {
            AuthError::Other(err_msg)
        }
    }
}
