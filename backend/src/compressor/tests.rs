#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::ascii_pixel::AsciiPixel;
    use crate::compressor::rle;
    use crate::compressor::gzip;
    use crate::compressor::compress_ascii_grid;
    use crate::compressor::decompress_ascii_grid;

    // Helper function to create test ASCII grids
    fn create_test_grid(width: usize, height: usize, pattern: &str) -> Vec<Vec<AsciiPixel>> {
        let chars: Vec<char> = pattern.chars().collect();
        let mut grid = Vec::new();

        for row in 0..height {
            let mut current_row = Vec::new();
            for col in 0..width {
                let char_index = (row * width + col) % chars.len();
                current_row.push(AsciiPixel {
                    ch: chars[char_index],
                    rgb: None,
                });
            }
            grid.push(current_row);
        }
        grid
    }

    fn create_colored_test_grid(width: usize, height: usize, pattern: &str) -> Vec<Vec<AsciiPixel>> {
        let chars: Vec<char> = pattern.chars().collect();
        let colors = vec![
            [255, 0, 0],   // Red
            [0, 255, 0],   // Green  
            [0, 0, 255],   // Blue
            [255, 255, 0], // Yellow
        ];
        
        let mut grid = Vec::new();
        
        for row in 0..height {
            let mut current_row = Vec::new();
            for col in 0..width {
                let char_index = (row * width + col) % chars.len();
                let color_index = (row * width + col) % colors.len();
                current_row.push(AsciiPixel {
                    ch: chars[char_index],
                    rgb: Some(colors[color_index]),
                });
            }
            grid.push(current_row);
        }
        grid
    }

    // Helper to create a specific colored grid for testing
    fn create_specific_colored_grid(pixels: Vec<(char, Option<[u8; 3]>)>) -> Vec<Vec<AsciiPixel>> {
        let mut grid = Vec::new();
        let mut row = Vec::new();
        
        for (ch, rgb) in pixels {
            row.push(AsciiPixel { ch, rgb });
        }
        grid.push(row);
        grid
    }

    // RLE Tests
    mod rle_tests {
        use super::*;

        #[test]
        fn test_rle_simple_compression() {
            let grid = create_test_grid(10, 1, "AAABBBCCCC");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            // Verify the compressed format
            assert_eq!(compressed.width, 10);
            assert_eq!(compressed.height, 1);
            assert_eq!(compressed.has_color, false);
            assert_eq!(compressed.rows.len(), 1);

            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 3, "Should have 3 RLE entries");

            // First entry: 3 A's
            assert_eq!(first_row[0].count, 3);
            assert_eq!(first_row[0].pixel.ch, 'A');
            assert_eq!(first_row[0].pixel.rgb, None);

            // Second entry: 3 B's
            assert_eq!(first_row[1].count, 3);
            assert_eq!(first_row[1].pixel.ch, 'B');
            assert_eq!(first_row[1].pixel.rgb, None);

            // Third entry: 4 C's
            assert_eq!(first_row[2].count, 4);
            assert_eq!(first_row[2].pixel.ch, 'C');
            assert_eq!(first_row[2].pixel.rgb, None);

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_no_compression() {
            let grid = create_test_grid(6, 1, "ABCDEF");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");

            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 6, "Should have 6 RLE entries");
            
            let expected_chars = ['A', 'B', 'C', 'D', 'E', 'F'];
            for (i, entry) in first_row.iter().enumerate() {
                assert_eq!(entry.count, 1);
                assert_eq!(entry.pixel.ch, expected_chars[i]);
                assert_eq!(entry.pixel.rgb, None);
            }

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_single_character() {
            // Test: 10 A's should compress to [(10, 'A')]
            let grid = create_test_grid(10, 1, "A");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");

            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 1, "Should have 1 RLE entry");
            assert_eq!(first_row[0].count, 10);
            assert_eq!(first_row[0].pixel.ch, 'A');
            assert_eq!(first_row[0].pixel.rgb, None);

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_multiple_rows() {
            // Test 2 rows: first row "AAA", second row "BBB"
            let grid = create_test_grid(3, 2, "AAABBB");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.rows.len(), 2);
            
            // First row: 3 A's
            assert_eq!(compressed.rows[0].len(), 1);
            assert_eq!(compressed.rows[0][0].count, 3);
            assert_eq!(compressed.rows[0][0].pixel.ch, 'A');
            
            // Second row: 3 B's  
            assert_eq!(compressed.rows[1].len(), 1);
            assert_eq!(compressed.rows[1][0].count, 3);
            assert_eq!(compressed.rows[1][0].pixel.ch, 'B');

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_with_colors() {
            let red = [255, 0, 0];
            let blue = [0, 0, 255];
            
            // Row: Red A, Red A, Blue B, Blue B
            let grid = create_specific_colored_grid(vec![
                ('A', Some(red)),
                ('A', Some(red)),
                ('B', Some(blue)),
                ('B', Some(blue)),
            ]);
            
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.has_color, true);
            assert_eq!(compressed.rows[0].len(), 2);
            
            // First entry: 2 red A's
            assert_eq!(compressed.rows[0][0].count, 2);
            assert_eq!(compressed.rows[0][0].pixel.ch, 'A');
            assert_eq!(compressed.rows[0][0].pixel.rgb, Some(red));
            
            // Second entry: 2 blue B's
            assert_eq!(compressed.rows[0][1].count, 2);
            assert_eq!(compressed.rows[0][1].pixel.ch, 'B');
            assert_eq!(compressed.rows[0][1].pixel.rgb, Some(blue));
        }

        #[test]
        fn test_rle_format_mixed_colors() {
            let red = [255, 0, 0];
            
            // Row: Red A, Red A, Uncolored A, Uncolored A
            // Should NOT compress across color boundaries
            let grid = create_specific_colored_grid(vec![
                ('A', Some(red)),
                ('A', Some(red)),
                ('A', None),
                ('A', None),
            ]);
            
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.rows[0].len(), 2, "Should have 2 entries (not compressed across color boundary)");
            
            // First entry: 2 red A's
            assert_eq!(compressed.rows[0][0].count, 2);
            assert_eq!(compressed.rows[0][0].pixel.ch, 'A');
            assert_eq!(compressed.rows[0][0].pixel.rgb, Some(red));
            
            // Second entry: 2 uncolored A's
            assert_eq!(compressed.rows[0][1].count, 2);
            assert_eq!(compressed.rows[0][1].pixel.ch, 'A');
            assert_eq!(compressed.rows[0][1].pixel.rgb, None);
        }

        #[test]
        fn test_rle_format_empty_grid() {
            let grid: Vec<Vec<AsciiPixel>> = vec![];
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.width, 0);
            assert_eq!(compressed.height, 0);
            assert_eq!(compressed.rows.len(), 0);
            assert_eq!(compressed.has_color, false);
        }

        #[test]
        fn test_rle_format_alternating_pattern() {
            // Test: "ABABAB" should compress to [(1, 'A'), (1, 'B'), (1, 'A'), (1, 'B'), (1, 'A'), (1, 'B')]
            let grid = create_test_grid(6, 1, "ABABAB");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 6, "Alternating pattern should not compress");
            
            for (i, entry) in first_row.iter().enumerate() {
                assert_eq!(entry.count, 1);
                assert_eq!(entry.pixel.ch, if i % 2 == 0 { 'A' } else { 'B' });
            }
        }

        #[test]
        fn test_rle_compression_efficiency() {
            // Test that RLE actually reduces the number of entries
            let grid = create_test_grid(100, 1, "A");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            // 100 identical characters should compress to 1 entry
            assert_eq!(compressed.rows[0].len(), 1);
            assert_eq!(compressed.rows[0][0].count, 100);
            
            // Verify round-trip works
            let decompressed = rle::decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(grid, decompressed);
        }

        #[test]
        fn test_rle_large_count() {
            // Test with very large run lengths
            let grid = create_test_grid(10000, 1, "X");
            let compressed = rle::compress_ascii_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.rows[0].len(), 1);
            assert_eq!(compressed.rows[0][0].count, 10000);
            assert_eq!(compressed.rows[0][0].pixel.ch, 'X');
        }
    }

    // Gzip Tests
    mod gzip_tests {
        use super::*;

        #[test]
        fn test_gzip_simple_compression() {
            let test_data = b"Hello, World! This is a test string for gzip compression.";
            let compressed = gzip::compress(test_data).expect("Gzip compression should succeed");
            let decompressed = gzip::decompress(&compressed).expect("Gzip decompression should succeed");
            
            assert_eq!(test_data, decompressed.as_slice());
        }

        #[test]
        fn test_gzip_empty_data() {
            let empty_data = b"";
            let compressed = gzip::compress(empty_data).expect("Gzip compression should succeed");
            let decompressed = gzip::decompress(&compressed).expect("Gzip decompression should succeed");
            
            assert_eq!(empty_data, decompressed.as_slice());
        }

        #[test]
        fn test_gzip_repetitive_data() {
            // Test data with lots of repetition (should compress well)
            let repetitive_data = "A".repeat(1000);
            let compressed = gzip::compress(repetitive_data.as_bytes()).expect("Gzip compression should succeed");
            let decompressed = gzip::decompress(&compressed).expect("Gzip decompression should succeed");
            
            assert_eq!(repetitive_data.as_bytes(), decompressed.as_slice());
            
            // Verify compression is effective
            assert!(compressed.len() < repetitive_data.len() / 2, "Gzip should compress repetitive data significantly");
        }

        #[test]
        fn test_gzip_random_data() {
            // Test with pseudo-random data (should not compress well)
            let random_data = "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()".repeat(20);
            let compressed = gzip::compress(random_data.as_bytes()).expect("Gzip compression should succeed");
            let decompressed = gzip::decompress(&compressed).expect("Gzip decompression should succeed");
            
            assert_eq!(random_data.as_bytes(), decompressed.as_slice());
        }

        #[test]
        fn test_gzip_large_data() {
            // Test with larger data set
            let large_data = "This is a test string that will be repeated many times to create a large dataset for testing gzip compression performance and correctness. ".repeat(100);
            let compressed = gzip::compress(large_data.as_bytes()).expect("Gzip compression should succeed");
            let decompressed = gzip::decompress(&compressed).expect("Gzip decompression should succeed");
            
            assert_eq!(large_data.as_bytes(), decompressed.as_slice());
        }

        #[test]
        fn test_gzip_ascii_art_pattern() {
            // Test with ASCII art-like patterns
            let ascii_art = "   ###   \n  #####  \n #######\n#########\n #######\n  #####  \n   ###   \n".repeat(10);
            let compressed = gzip::compress(ascii_art.as_bytes()).expect("Gzip compression should succeed");
            let decompressed = gzip::decompress(&compressed).expect("Gzip decompression should succeed");
            
            assert_eq!(ascii_art.as_bytes(), decompressed.as_slice());
            
            // ASCII art should compress reasonably well
            assert!(compressed.len() < ascii_art.len(), "ASCII art should compress");
        }

        #[test]
        fn test_gzip_compression_ratio() {
            let test_data = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
            let compressed = gzip::compress(test_data.as_bytes()).expect("Gzip compression should succeed");
            
            let ratio = gzip::compression_ratio(test_data.as_bytes(), &compressed);
            assert!(ratio > 0.0 && ratio < 1.0, "Compression ratio should be between 0 and 1");
        }
    }

    // Combined RLE + Gzip Tests
    mod combined_compression_tests {
        use super::*;

        #[test]
        fn test_rle_then_gzip_compression() {
            // Test the two-stage compression pipeline
            let grid = create_test_grid(100, 10, "AAAAAABBBBBBCCCCCCDDDDDD      ");
            
            // Stage 1: RLE compression
            let rle_compressed = rle::compress_ascii_grid(&grid).expect("RLE compression should succeed");
            
            // Stage 2: Gzip compression of RLE data
            let serialized_rle = gzip::serialize_compressed_grid(&rle_compressed).expect("Serialization should succeed");
            let gzip_compressed = gzip::compress(&serialized_rle).expect("Gzip compression should succeed");
            
            // Decompression pipeline
            let gzip_decompressed = gzip::decompress(&gzip_compressed).expect("Gzip decompression should succeed");
            let rle_data = gzip::deserialize_compressed_grid(&gzip_decompressed).expect("Deserialization should succeed");
            let final_grid = rle::decompress_ascii_grid(&rle_data).expect("RLE decompression should succeed");
            
            assert_eq!(grid, final_grid, "Combined compression/decompression should preserve original data");
        }

        #[test]
        fn test_combined_compression_efficiency() {
            // Test that RLE + Gzip provides better compression than either alone
            let grid = create_test_grid(200, 20, "████████        ████████        ");
            
            // Original size (approximation)
            let original_size = grid.len() * grid[0].len() * 5; // Rough estimate for AsciiPixel serialization
            
            // RLE only
            let rle_compressed = rle::compress_ascii_grid(&grid).expect("RLE compression should succeed");
            let rle_serialized = gzip::serialize_compressed_grid(&rle_compressed).expect("Serialization should succeed");
            
            // Gzip only (on original data)
            let original_serialized = gzip::serialize_grid(&grid).expect("Grid serialization should succeed");
            let gzip_only = gzip::compress(&original_serialized).expect("Gzip compression should succeed");
            
            // Combined RLE + Gzip
            let combined = gzip::compress(&rle_serialized).expect("Combined compression should succeed");
            
            // Combined should be smaller than gzip-only for this repetitive pattern
            assert!(combined.len() <= gzip_only.len(), "Combined compression should be at least as good as gzip-only");
            
            // Verify compression is significant
            assert!(combined.len() < original_size / 2, "Combined compression should significantly reduce size");
        }

        #[test]
        fn test_combined_with_colors() {
            let grid = create_colored_test_grid(50, 10, "██  ██  ");
            
            // Combined compression
            let rle_compressed = rle::compress_ascii_grid(&grid).expect("RLE compression should succeed");
            let serialized = gzip::serialize_compressed_grid(&rle_compressed).expect("Serialization should succeed");
            let final_compressed = gzip::compress(&serialized).expect("Gzip compression should succeed");
            
            // Decompression
            let decompressed_gzip = gzip::decompress(&final_compressed).expect("Gzip decompression should succeed");
            let rle_data = gzip::deserialize_compressed_grid(&decompressed_gzip).expect("Deserialization should succeed");
            let final_grid = rle::decompress_ascii_grid(&rle_data).expect("RLE decompression should succeed");
            
            assert_eq!(grid, final_grid, "Combined compression should preserve colored data");
            assert_eq!(rle_data.has_color, true, "Color information should be preserved");
        }

        #[test]
        fn test_combined_alternating_pattern() {
            // Test with a pattern that doesn't compress well with RLE
            let grid = create_test_grid(100, 10, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            
            let rle_compressed = rle::compress_ascii_grid(&grid).expect("RLE compression should succeed");
            let serialized = gzip::serialize_compressed_grid(&rle_compressed).expect("Serialization should succeed");
            let final_compressed = gzip::compress(&serialized).expect("Gzip compression should succeed");
            
            // Decompression
            let decompressed_gzip = gzip::decompress(&final_compressed).expect("Gzip decompression should succeed");
            let rle_data = gzip::deserialize_compressed_grid(&decompressed_gzip).expect("Deserialization should succeed");
            let final_grid = rle::decompress_ascii_grid(&rle_data).expect("RLE decompression should succeed");
            
            assert_eq!(grid, final_grid, "Combined compression should work even with non-repetitive patterns");
        }

        #[test]
        fn test_compression_metadata_preservation() {
            let grid = create_colored_test_grid(30, 20, "▓▓▓   ");
            
            let rle_compressed = rle::compress_ascii_grid(&grid).expect("RLE compression should succeed");
            let original_width = rle_compressed.width;
            let original_height = rle_compressed.height;
            let original_has_color = rle_compressed.has_color;
            
            // Full compression/decompression cycle
            let serialized = gzip::serialize_compressed_grid(&rle_compressed).expect("Serialization should succeed");
            let gzip_compressed = gzip::compress(&serialized).expect("Gzip compression should succeed");
            let gzip_decompressed = gzip::decompress(&gzip_compressed).expect("Gzip decompression should succeed");
            let rle_data = gzip::deserialize_compressed_grid(&gzip_decompressed).expect("Deserialization should succeed");
            
            // Verify metadata is preserved
            assert_eq!(rle_data.width, original_width, "Width should be preserved");
            assert_eq!(rle_data.height, original_height, "Height should be preserved");
            assert_eq!(rle_data.has_color, original_has_color, "Color flag should be preserved");
        }

        #[test]
        fn test_large_grid_combined_compression() {
            // Test with a large grid to ensure scalability
            let grid = create_test_grid(500, 100, "█████     █████     ");
            
            let rle_compressed = rle::compress_ascii_grid(&grid).expect("RLE compression should succeed");
            let serialized = gzip::serialize_compressed_grid(&rle_compressed).expect("Serialization should succeed");
            let final_compressed = gzip::compress(&serialized).expect("Gzip compression should succeed");
            
            // Verify significant compression
            let estimated_original_size = grid.len() * grid[0].len() * 5;
            assert!(final_compressed.len() < estimated_original_size / 10, "Large repetitive grid should compress very well");
            
            // Verify decompression works
            let decompressed_gzip = gzip::decompress(&final_compressed).expect("Gzip decompression should succeed");
            let rle_data = gzip::deserialize_compressed_grid(&decompressed_gzip).expect("Deserialization should succeed");
            let final_grid = rle::decompress_ascii_grid(&rle_data).expect("RLE decompression should succeed");
            
            assert_eq!(grid, final_grid, "Large grid should decompress correctly");
        }
    }

    // Integration tests
    mod integration_tests {
        use super::*;

        #[test]
        fn test_typical_ascii_art_pattern() {
            // Test a pattern that's common in ASCII art
            let grid = create_test_grid(50, 10, "   ###   @@@   ");
            let compressed = compress_ascii_grid(&grid).expect("Compression should succeed");
            let decompressed = decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(grid, decompressed);
        }

        #[test]
        fn test_colored_ascii_art() {
            let grid = create_colored_test_grid(20, 5, "█▓▒░");
            let compressed = compress_ascii_grid(&grid).expect("Compression should succeed");
            let decompressed = decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(grid, decompressed);
        }

        #[test]
        fn test_mixed_patterns() {
            let grid = create_test_grid(30, 15, "AAAAABBBBCCCCCDDDDDEEEEE     ");
            let compressed = compress_ascii_grid(&grid).expect("Compression should succeed");
            let decompressed = decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(grid, decompressed);
        }

        #[test]
        fn test_end_to_end_compression_pipeline() {
            // Test the complete pipeline from grid to final compressed format
            let grid = create_test_grid(100, 50, "██████      ██████      ");
            
            // Use the main compression interface
            let compressed = compress_ascii_grid(&grid).expect("Compression should succeed");
            let decompressed = decompress_ascii_grid(&compressed).expect("Decompression should succeed");
            
            assert_eq!(grid, decompressed, "End-to-end pipeline should preserve data");
        }
    }
}