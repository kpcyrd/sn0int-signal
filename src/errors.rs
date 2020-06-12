pub use anyhow::{bail, format_err, Context, Result};
use std::fmt;

#[derive(Debug)]
pub struct WebError(anyhow::Error);

impl fmt::Display for WebError {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        writeln!(w, "{}", self.0)
    }
}

impl From<anyhow::Error> for WebError {
    fn from(err: anyhow::Error) -> WebError {
        WebError(err)
    }
}

impl actix_web::error::ResponseError for WebError {}
