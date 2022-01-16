use std::{error, fmt};
use std::any::TypeId;
use std::error::Error;
use std::fmt::Formatter;

type Result<T> = std::result::Result<T, PlayerPurchasesError>;

#[derive(Debug)]
pub enum PlayerPurchasesError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl fmt::Display for PlayerPurchasesError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self{
            PlayerPurchasesError::Reqwest(e) => {
                write!(f,"{}",e)
            }
            PlayerPurchasesError::SerdeJson(e) => {
                write!(f,"{}",e)
            }
        }
    }
}

impl error::Error for PlayerPurchasesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PlayerPurchasesError::Reqwest(e) => {
                Some(e)
            }
            PlayerPurchasesError::SerdeJson(e) => {
                Some(e)
            }
        }
    }
}

impl From<reqwest::Error> for PlayerPurchasesError {
    fn from(err: reqwest::Error) -> Self {
        PlayerPurchasesError::Reqwest(err)
    }
}

impl From<serde_json::Error> for PlayerPurchasesError {
    fn from(err: serde_json::Error) -> Self {
        PlayerPurchasesError::SerdeJson(err)
    }
}