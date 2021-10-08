use thiserror::Error;

/// Enum listing possible authentication error codes.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// An error occured when connecting to or using the database.
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),

    /// An error occured with the Argon2id hashing implementation.
    #[error("hashing error")]
    HashingError,

    /// If the request was invalid or malformed.
    #[error("the request was invalid {0}")]
    InvalidRequest(String),

    /// An error occured when validating or generating a JWT.
    #[error("invalid token")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    /// If the username and password combination did not match when attempting to authenticate.
    #[error("invalid username or password")]
    InvalidUsernameOrPassword,

    /// Any other, unknown error sources.
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error + Sync + Send>),

    /// If a registration was attempted, but the email address already exists in the database.
    #[error("a user with the email {0} already exists")]
    UserAlreadyExists(String),
}

/// TODO: documentation
impl From<AuthError> for tonic::Status {
    fn from(auth_error: AuthError) -> tonic::Status {
        match auth_error {
            AuthError::DatabaseError(_) => tonic::Status::unavailable(format!("{:?}", auth_error)),
            AuthError::HashingError => tonic::Status::unavailable(format!("{:?}", auth_error)),
            AuthError::InvalidRequest(_) => {
                tonic::Status::invalid_argument(format!("{:?}", auth_error))
            }
            AuthError::InvalidToken(_) => {
                tonic::Status::unauthenticated(format!("{:?}", auth_error))
            }
            AuthError::InvalidUsernameOrPassword => {
                tonic::Status::unauthenticated(format!("{:?}", auth_error))
            }
            AuthError::Unknown(_) => tonic::Status::unknown(format!("{:?}", auth_error)),
            AuthError::UserAlreadyExists(_) => {
                tonic::Status::invalid_argument(format!("{:?}", auth_error))
            }
        }
    }
}
