#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::AsciiPixel;

    // Helper function to create test ASCII grids
    fn create_test_grid(width: usize, height: usize, pattern: &str) -> Vec<Vec<AsciiPixel>> {
        let chars: Vec<char> = pattern.chars().collect();
        let mut grid = Vec::new();

        for row in 0..height {
            let mut current_row = Vec::new();
            for col in 0..width {
                let char_index = (row * width + col) % chars.len();
                current_row.push(AsciiPixel {
                    character: chars[char_index],
                    color: None,
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
                    character: chars[char_index],
                    color: Some(colors[color_index]),
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
        
        for (character, color) in pixels {
            row.push(AsciiPixel { character, color });
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
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            // Verify the compressed format
            assert_eq!(compressed.width, 10);
            assert_eq!(compressed.height, 1);
            assert_eq!(compressed.has_color, false);
            assert_eq!(compressed.rows.len(), 1);

            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 3, "Should have 3 RLE entries");

            // First entry: 3 A's
            assert_eq!(first_row[0].count, 3);
            assert_eq!(first_row[0].pixel.character, 'A');
            assert_eq!(first_row[0].pixel.color, None);

            // Second entry: 3 B's
            assert_eq!(first_row[1].count, 3);
            assert_eq!(first_row[1].pixel.character, 'B');
            assert_eq!(first_row[1].pixel.color, None);

            // Third entry: 4 C's
            assert_eq!(first_row[2].count, 4);
            assert_eq!(first_row[2].pixel.character, 'C');
            assert_eq!(first_row[2].pixel.color, None);

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_no_compression() {
            let grid = create_test_grid(6, 1, "ABCDEF");
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");

            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 6, "Should have 6 RLE entries");
            
            let expected_chars = ['A', 'B', 'C', 'D', 'E', 'F'];
            for (i, entry) in first_row.iter().enumerate() {
                assert_eq!(entry.count, 1);
                assert_eq!(entry.pixel.character, expected_chars[i]);
                assert_eq!(entry.pixel.color, None);
            }

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_single_character() {
            // Test: 10 A's should compress to [(10, 'A')]
            let grid = create_test_grid(10, 1, "A");
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");

            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 1, "Should have 1 RLE entry");
            assert_eq!(first_row[0].count, 10);
            assert_eq!(first_row[0].pixel.character, 'A');
            assert_eq!(first_row[0].pixel.color, None);

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(decompressed, grid, "Decompressed grid should match original grid");
        }

        #[test]
        fn test_rle_format_multiple_rows() {
            // Test 2 rows: first row "AAA", second row "BBB"
            let grid = create_test_grid(3, 2, "AAABBB");
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.rows.len(), 2);
            
            // First row: 3 A's
            assert_eq!(compressed.rows[0].len(), 1);
            assert_eq!(compressed.rows[0][0].count, 3);
            assert_eq!(compressed.rows[0][0].pixel.character, 'A');
            
            // Second row: 3 B's  
            assert_eq!(compressed.rows[1].len(), 1);
            assert_eq!(compressed.rows[1][0].count, 3);
            assert_eq!(compressed.rows[1][0].pixel.character, 'B');

            // Verify the original grid can be reconstructed
            let decompressed = rle::decompress_grid(&compressed).expect("Decompression should succeed");
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
            
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.has_color, true);
            assert_eq!(compressed.rows[0].len(), 2);
            
            // First entry: 2 red A's
            assert_eq!(compressed.rows[0][0].count, 2);
            assert_eq!(compressed.rows[0][0].pixel.character, 'A');
            assert_eq!(compressed.rows[0][0].pixel.color, Some(red));
            
            // Second entry: 2 blue B's
            assert_eq!(compressed.rows[0][1].count, 2);
            assert_eq!(compressed.rows[0][1].pixel.character, 'B');
            assert_eq!(compressed.rows[0][1].pixel.color, Some(blue));
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
            
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.rows[0].len(), 2, "Should have 2 entries (not compressed across color boundary)");
            
            // First entry: 2 red A's
            assert_eq!(compressed.rows[0][0].count, 2);
            assert_eq!(compressed.rows[0][0].pixel.character, 'A');
            assert_eq!(compressed.rows[0][0].pixel.color, Some(red));
            
            // Second entry: 2 uncolored A's
            assert_eq!(compressed.rows[0][1].count, 2);
            assert_eq!(compressed.rows[0][1].pixel.character, 'A');
            assert_eq!(compressed.rows[0][1].pixel.color, None);
        }

        #[test]
        fn test_rle_format_empty_grid() {
            let grid: Vec<Vec<AsciiPixel>> = vec![];
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.width, 0);
            assert_eq!(compressed.height, 0);
            assert_eq!(compressed.rows.len(), 0);
            assert_eq!(compressed.has_color, false);
        }

        #[test]
        fn test_rle_format_alternating_pattern() {
            // Test: "ABABAB" should compress to [(1, 'A'), (1, 'B'), (1, 'A'), (1, 'B'), (1, 'A'), (1, 'B')]
            let grid = create_test_grid(6, 1, "ABABAB");
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            let first_row = &compressed.rows[0];
            assert_eq!(first_row.len(), 6, "Alternating pattern should not compress");
            
            for (i, entry) in first_row.iter().enumerate() {
                assert_eq!(entry.count, 1);
                assert_eq!(entry.pixel.character, if i % 2 == 0 { 'A' } else { 'B' });
            }
        }

        #[test]
        fn test_rle_compression_efficiency() {
            // Test that RLE actually reduces the number of entries
            let grid = create_test_grid(100, 1, "A");
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            // 100 identical characters should compress to 1 entry
            assert_eq!(compressed.rows[0].len(), 1);
            assert_eq!(compressed.rows[0][0].count, 100);
            
            // Verify round-trip works
            let decompressed = rle::decompress_grid(&compressed).expect("Decompression should succeed");
            assert_eq!(grid, decompressed);
        }

        #[test]
        fn test_rle_large_count() {
            // Test with very large run lengths
            let grid = create_test_grid(10000, 1, "X");
            let compressed = rle::compress_grid(&grid).expect("Compression should succeed");
            
            assert_eq!(compressed.rows[0].len(), 1);
            assert_eq!(compressed.rows[0][0].count, 10000);
            assert_eq!(compressed.rows[0][0].pixel.character, 'X');
        }
    }

    // Additional tests using helper functions...
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
    }
}