
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Out of Memory")]
    OutOfMemory,
}

pub type Result<T> = std::result::Result<T, Error>;