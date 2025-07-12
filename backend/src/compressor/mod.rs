pub mod rle;
pub mod gzip;

#[cfg(test)]
mod tests;

// Main compression interface - RLE + Gzip pipeline
pub fn compress_ascii_grid(grid: &[Vec<crate::converter::ascii_pixel::AsciiPixel>]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Stage 1: RLE compression
    let rle_compressed = rle::compress_grid(grid)?;
    
    // Stage 2: Serialize RLE data
    let serialized = gzip::serialize_compressed_grid(&rle_compressed)?;
    
    // Stage 3: Gzip compression
    let final_compressed = gzip::compress(&serialized)?;
    
    Ok(final_compressed)
}