/// Error type for the ASCII image converter.
#[derive(Debug)]
pub enum ConverterError {
    /// Error originating from the `image` crate (e.g., decoding, format issues).
    ImageError(image::ImageError),
    /// Error for invalid configuration or function parameters.
    InvalidParameter(String),
}

/// Allow automatic conversion from `image::ImageError` to `ConverterError`.
impl From<image::ImageError> for ConverterError {
    fn from(err: image::ImageError) -> Self {
        ConverterError::ImageError(err)
    }
}

/// Implements user-friendly display for `ConverterError`.
impl std::fmt::Display for ConverterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConverterError::ImageError(err) => write!(f, "Image error: {}", err),
            ConverterError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

/// Implements the standard error trait for `ConverterError`.
impl std::error::Error for ConverterError {}