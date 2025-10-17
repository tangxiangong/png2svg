#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid File Path")]
    InvalidFilePath,
    #[error(transparent)]
    TransparentError(#[from] image::ImageError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
