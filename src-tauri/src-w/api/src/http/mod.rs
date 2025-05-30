use scraper::error::SelectorErrorKind;
use thiserror::Error;

pub mod dl_site;

pub type HttpResult<T> = Result<T, HttpError>;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Invalid request: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse HTML: {0}")]
    Scraper(String),
}

impl From<SelectorErrorKind<'_>> for HttpError {
    fn from(err: SelectorErrorKind) -> Self {
        HttpError::Scraper(format!("{}", err))
    }
}
