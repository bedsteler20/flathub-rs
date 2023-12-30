use reqwest_middleware::Error as MiddlewareError;
use reqwest::Error as ReqwestError;


#[derive(Debug)]
pub enum Error {
    Middleware(anyhow::Error),
    Requests(reqwest::Error),
}

impl From<MiddlewareError> for Error {
    fn from(e: MiddlewareError) -> Self {
        match e {
            MiddlewareError::Reqwest(e) => Error::Requests(e),
            MiddlewareError::Middleware(e) => Error::Middleware(e),
            
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Error::Requests(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Middleware(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match self {
           Error::Middleware(e) => write!(f, "{}", e),
           Error::Requests(e) => write!(f, "{}", e),
       }
    }
}
