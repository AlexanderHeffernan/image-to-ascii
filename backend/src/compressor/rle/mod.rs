mod types;
mod compress;

#[cfg(test)]
mod decompress;

pub use types::CompressedGrid;
pub use compress::compress_grid;

#[cfg(test)]
pub use decompress::decompress_grid;