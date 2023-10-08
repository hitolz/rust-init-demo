use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::api::error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),


    #[error("ToStrError errror: {0}")]
    ToStrError(#[from] reqwest::header::ToStrError),

    #[error("SerdeJsonError errror: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("ParseError errror: {0}")]
    ParseError(#[from] chrono::ParseError),

    #[error("ReqwestError errror: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("KafkaError: {0}")]
    KafkaError(#[from] rdkafka::error::KafkaError),
 
    #[error("{0}")]
    ApiError(String),

    #[error("无效参数: {0}")]
    InvalidParam(String),

   #[error("商户名称已存在")]
    DuplicateMerchantName,

    #[error("{0}")]
    BizError(String),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        error(Some(self.to_string()))
    }
}

pub struct BizError{
    pub message: String,
}

impl BizError {
    pub fn new(message: String) -> Self {
        BizError {
            message
        }
    }
}

impl Debug for BizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BizError: {}", self.message)
    }
}

impl Display for BizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BizError: {}", self.message)
    }
}

impl std::error::Error for BizError{
    fn description(&self) -> &str {
        self.message.as_str()
    }
}
unsafe impl Send for BizError {}
unsafe impl Sync for BizError {}