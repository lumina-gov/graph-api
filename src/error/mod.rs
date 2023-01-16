use juniper::{ScalarValue, graphql_value, FieldError, IntoFieldError};

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
            // These codes should never change
            // as they are used by the frontend to handle errors
            graphql_value!({
                "code": match self {
                    ErrorCode::UserAlreadyExists => "USER_ALREADY_EXISTS",
                    ErrorCode::PasswordMismatch => "PASSWORD_MISMATCH",
                    ErrorCode::UserNotFound => "USER_NOT_FOUND",
                    ErrorCode::InvalidToken => "INVALID_TOKEN",
                    ErrorCode::CouldNotCreateToken => "COULD_NOT_CREATE_TOKEN",
                    ErrorCode::FailedToHashPassword => "FAILED_TO_HASH_PASSWORD",
                    ErrorCode::Unauthenticated => "UNAUTHENTICATED",
                }
            })
        )
    }
}

impl Into<FieldError> for ErrorCode {
    fn into(self) -> FieldError {
        self.into_field_error()
    }
}