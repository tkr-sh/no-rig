use {
    axum::response::{IntoResponse, Response},
    hyper::{
        StatusCode,
        header::{InvalidHeaderValue, ToStrError},
    },
    std::{convert::Infallible, str::Utf8Error},
};

#[derive(Debug)]
pub enum ServerError {
    Status(hyper::StatusCode),
    Infaillible(Infallible),
    Utf8Error(Utf8Error),
    InvalidHeader(InvalidHeaderValue),
    DebugedError(String),
    PublicRessourceNotFound(String),
    ToStrError(ToStrError),
    SqlError(sqlx::Error),
}

/// Macro to easily implement errors into
macro_rules! impl_from_error {
    ($from:ty, $to:path) => {
        impl From<$from> for ServerError {
            fn from(rejection: $from) -> Self {
                $to(rejection)
            }
        }
    };
}

impl_from_error!(hyper::StatusCode, Self::Status);
impl_from_error!(Infallible, Self::Infaillible);
impl_from_error!(Utf8Error, Self::Utf8Error);
impl_from_error!(String, Self::DebugedError);
impl_from_error!(InvalidHeaderValue, Self::InvalidHeader);
impl_from_error!(ToStrError, Self::ToStrError);
impl_from_error!(sqlx::Error, Self::SqlError);

pub type ServerResult<T> = Result<T, ServerError>;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        eprintln!("{self:#?}");
        let err_msg = match self {
            Self::InvalidHeader(err) => {
                format!("Unexpected header value: {err}")
            },
            Self::DebugedError(err) => {
                format!("Unexpected error: {err}")
            },
            Self::Infaillible(err) => {
                format!("This error should not be possible: {err:#?}")
            },
            Self::ToStrError(err) => {
                format!("Invalid str: {err}")
            },
            Self::Utf8Error(err) => {
                format!("Error decoding buffer to UTF-8: {err:#?}")
            },
            Self::SqlError(err) => {
                format!("SQL error: {err:#?}")
            },
            Self::PublicRessourceNotFound(path) => {
                return (StatusCode::NOT_FOUND, format!("Couldn't find file: {path}"))
                    .into_response();
            },
            Self::Status(status_code) => return status_code.into_response(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response()
    }
}


/// A trait for handling `Result` types by exiting the program with a custom error message if an error occurs.
///
/// This trait provides a convenient way to handle errors in situations where encountering an error
/// should result in immediate program termination with a meaningful error message.
///
/// # Example
///
/// ```ignore
/// use wini::shared::wini::err::ExitWithMessageIfErr;
///
/// fn main() {
///     // Will exit with error message if file cannot be opened
///     let file = std::fs::File::open("config.txt")
///         .exit_with_msg_if_err("Failed to open configuration file");
///
///     // Continue processing with file...
/// }
/// ```
///
/// # Panics
///
/// This trait implementation never panics. Instead, it exits the program with status code 1
/// when encountering an error.
pub trait ExitWithMessageIfErr<T> {
    /// Handles a `Result` by either returning the success value or exiting the program
    /// with a custom error message if an error occurs.
    fn exit_with_msg_if_err(self, msg: impl std::fmt::Display) -> T;
    fn exit_with_msg_to_compute_if_err<S: std::fmt::Display, F: Fn() -> S>(self, msg: F) -> T;
}

impl<T, E> ExitWithMessageIfErr<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn exit_with_msg_if_err(self, msg: impl std::fmt::Display) -> T {
        self.map_err(|err| {
            log::error!("{msg}: {err:?}");
            std::process::exit(1);
        })
        .expect("Already exited if `Err`")
    }

    fn exit_with_msg_to_compute_if_err<S: std::fmt::Display, F: Fn() -> S>(self, msg: F) -> T {
        self.map_err(|err| {
            log::error!("{}: {err:?}", msg());
            std::process::exit(1);
        })
        .unwrap()
    }
}

