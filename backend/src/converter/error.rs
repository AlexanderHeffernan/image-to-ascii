#[derive(Debug)]
pub enum ConverterError {
    ImageError(image::ImageError),
    InvalidParameter(String),
}

impl From<image::ImageError> for ConverterError {
    fn from(err: image::ImageError) -> Self {
        ConverterError::ImageError(err)
    }
}

impl std::fmt::Display for ConverterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConverterError::ImageError(err) => write!(f, "Image error: {}", err),
            ConverterError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for ConverterError {}