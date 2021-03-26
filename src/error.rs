//! Merkle-Mountain-Range errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("generic store error: `{0}`")]
    Store(String),
}
