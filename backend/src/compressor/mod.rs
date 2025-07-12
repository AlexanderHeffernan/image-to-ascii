pub mod rle;
pub mod gzip;
pub mod error;

// Re-export commonly used items
pub use rle::*;
pub use error::*;

#[cfg(test)]
mod tests;