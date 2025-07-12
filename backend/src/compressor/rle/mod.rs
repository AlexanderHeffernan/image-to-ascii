mod types;
mod compress;
mod decompress;

pub use types::{RleEntry, CompressedGrid, CompressionError};
pub use compress::{compress_grid};
pub use decompress::{decompress_grid};