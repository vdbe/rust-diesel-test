#[allow(dead_code)]
use serde::{de, ser};
use std::fmt;

pub type Result<T> = std::result::Result<T, CustomError>;

#[derive(Debug)]
pub enum CustomError {
    Serialize(String),

    Deserialize(String),

    Diesel(diesel::result::Error),

    R2d2(r2d2::Error),

    /// Filler for not implmented error types
    Message(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialize(_) => todo!(),
            Self::Deserialize(_) => todo!(),
            Self::Diesel(_) => todo!(),
            Self::R2d2(_) => todo!(),
            Self::Message(msg) => write!(f, "Message: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}

impl ser::Error for CustomError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Serialize(msg.to_string())
    }
}

impl de::Error for CustomError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Deserialize(msg.to_string())
    }
}

impl From<diesel::result::Error> for CustomError {
    fn from(e: diesel::result::Error) -> Self {
        Self::Diesel(e)
    }
}

impl From<r2d2::Error> for CustomError {
    fn from(e: r2d2::Error) -> Self {
        Self::R2d2(e)
    }
}

impl From<std::num::TryFromIntError> for CustomError {
    fn from(e: std::num::TryFromIntError) -> Self {
        Self::Message(e.to_string())
    }
}
