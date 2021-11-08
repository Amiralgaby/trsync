use std::{fmt, io};

use crate::types::{AbsoluteFilePath, ContentId, RevisionId};

#[derive(Debug)]
pub enum ClientError {
    InputFileError(AbsoluteFilePath),
    RequestError(String),
    UnexpectedResponse(String),
    AlreadyExistResponse(ContentId, RevisionId),
    AlreadyExistResponseAndFailToFoundIt(String),
    NotFoundResponse(String),
    DecodingResponseError(String),
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(format!("Error happen when make request : {:?}", error))
    }
}

impl From<Error> for ClientError {
    fn from(error: Error) -> Self {
        Self::RequestError(format!("Error happen when make request : {:?}", error))
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ClientError::InputFileError(absolute_file_path) => {
                format!("Error when reading input file '{}'", absolute_file_path)
            }
            ClientError::RequestError(message) => {
                format!("Error when making request : {}", message)
            }
            ClientError::UnexpectedResponse(message) => {
                format!("UnExpected response : {}", message)
            }
            ClientError::AlreadyExistResponse(content_id, revision_id) => {
                format!("Content already exist : {}({})", content_id, revision_id)
            }
            ClientError::AlreadyExistResponseAndFailToFoundIt(message) => format!(
                "Already exist but fail to found remote content : {}",
                message
            ),
            ClientError::NotFoundResponse(message) => format!("Not found : {}", message),
            ClientError::DecodingResponseError(message) => format!("Decoding error : {}", message),
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum Error {
    FailToCreateContentOnRemote(String),
    FailToCreateContentOnLocal(String),
    UnIndexedRelativePath(String),
    UnexpectedError(String),
    PathCastingError(String),
    PathManipulationError(String),
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Self {
        Error::UnexpectedError(format!("{:?}", err))
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::UnexpectedError(format!("{:?}", error))
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::UnexpectedError(format!("{:?}", error))
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Error::UnexpectedError(format!("{:?}", error))
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::UnexpectedError(format!("Unable to strip prefix {:?}", error))
    }
}

impl From<notify::Error> for Error {
    fn from(error: notify::Error) -> Self {
        Error::UnexpectedError(format!("Notify error {:?}", error))
    }
}
