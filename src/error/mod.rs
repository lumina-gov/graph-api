use juniper::{ScalarValue, graphql_value};


#[derive(Debug)]
pub enum ErrorCode {
    UserAlreadyExists,
    PasswordMismatch,
    UserNotFound,
    InvalidToken,
    CouldNotCreateToken,
    FailedToHashPassword,
    Unauthenticated,
}

/// These error codes should never change,
/// so we can reliably match on them in the frontend.
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::UserAlreadyExists => write!(f, "USER_ALREADY_EXISTS"),
            ErrorCode::PasswordMismatch => write!(f, "PASSWORD_MISMATCH"),
            ErrorCode::UserNotFound => write!(f, "USER_NOT_FOUND"),
            ErrorCode::InvalidToken => write!(f, "INVALID_TOKEN"),
            ErrorCode::CouldNotCreateToken => write!(f, "COULD_NOT_CREATE_TOKEN"),
            ErrorCode::FailedToHashPassword => write!(f, "FAILED_TO_HASH_PASSWORD"),
            ErrorCode::Unauthenticated => write!(f, "UNAUTHENTICATED"),
        }
    }
}

impl<S: ScalarValue> juniper::IntoFieldError<S> for ErrorCode {
    fn into_field_error(self) -> juniper::FieldError<S> {
        juniper::FieldError::new(
            match &self {
                Self::UserAlreadyExists => "User already exists",
                Self::PasswordMismatch => "Wrong password",
                Self::UserNotFound => "User not found",
                Self::InvalidToken => "Invalid authentication token",
                Self::CouldNotCreateToken => "Could not create authentication token",
                Self::FailedToHashPassword => "Could not hash password",
                Self::Unauthenticated => "You are not authenticated",
            },
            graphql_value!({
                "code": self.to_string(),
            })
        )
    }
}