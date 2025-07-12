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

pub fn decompress_ascii_grid(compressed_data: &[u8]) -> Result<Vec<Vec<crate::converter::ascii_pixel::AsciiPixel>>, Box<dyn std::error::Error>> {
    // Stage 1: Gzip decompression
    let gzip_decompressed = gzip::decompress(compressed_data)?;
    
    // Stage 2: Deserialize RLE data
    let rle_data = gzip::deserialize_compressed_grid(&gzip_decompressed)?;
    
    // Stage 3: RLE decompression
    let final_grid = rle::decompress_grid(&rle_data)?;
    
    Ok(final_grid)
}