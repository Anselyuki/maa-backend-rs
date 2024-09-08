use std::borrow::Cow;
use std::fmt::Write;

use http::StatusCode;

use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MaaError {
    /**
     * Internal errors
     */

    #[error("Error getting env var: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("Error serializing struct: {0}")]
    SerializeError(#[from] bson::ser::Error),

    #[error("Error doing database operations: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("No default database found")]
    NoDefaultDBError,

    #[error("Error doing redis operations: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Error getting redis connection: {0}")]
    RedisPoolError(#[from] bb8::RunError<redis::RedisError>),

    #[error("Error hashing password: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Error parsing int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Jwt error: {0}")]
    JsonWebTokensError(#[from] jsonwebtokens::error::Error),

    #[error("Mail send error: {0}")]
    MailSendError(#[from] mail_send::Error),

    #[error("Error registing handlebars template: {0}")]
    TemplateError(#[from] handlebars::TemplateError),

    #[error("Error rendering handlebars template: {0}")]
    RenderError(#[from] handlebars::RenderError),

    /**
     * Business errors
     */

    #[error("用户不存在或密码错误")]
    LoginFail,

    #[error("用户未启用")]
    UserNotEnabled,

    #[error("JWT验证失败")]
    JwtVerifyFailed,

    #[error("用户id不存在")]
    NoneUserId,

    #[error("Validate失败: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("验证码发送过于频繁")]
    VCodeSentTooFrequently,

    #[error("用户已存在")]
    RegistrationUserExist,

    #[error("验证码不匹配")]
    VCodeNotMatch,
}

impl IntoResponse for MaaError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match &self {
            MaaError::LoginFail => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::UserNotEnabled => Response::builder()
                .status(10003)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::NoneUserId => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::ValidationError(errors) => {
                let field_errors = errors.field_errors();
                let mut error_msg = String::new();
                for (field, errors) in field_errors {
                    for error in errors {
                        let message = match error.message {
                            Some(ref msg) => match msg {
                                Cow::Borrowed(msg) => msg.to_owned(),
                                Cow::Owned(msg) => msg,
                            },
                            None => "Validation failed",
                        };
                        if let Err(e) =
                            writeln!(error_msg, "{}: {}", field, message)
                        {
                            tracing::error!(
                                "Error writing error message: {}",
                                e
                            );
                        }
                    }
                }
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(error_msg.into())
                    .unwrap_or_default()
            }
            MaaError::VCodeSentTooFrequently => Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::RegistrationUserExist => Response::builder()
                .status(10004)
                .body(self.to_string().into())
                .unwrap_or_default(),
            MaaError::VCodeNotMatch => Response::builder()
                .status(401)
                .body(self.to_string().into())
                .unwrap_or_default(),
            _ => {
                tracing::error!("{}", self);
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
