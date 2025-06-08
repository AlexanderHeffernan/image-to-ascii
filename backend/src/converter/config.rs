use serde::{Serialize, Deserialize};

// ===== Default Value Functions =====
fn default_charset()                -> Vec<char>    { vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'] }
fn default_output_width()           -> u32          { 200 }
fn default_output_height()          -> Option<u32>  { None }
fn default_brightness()             -> f32          { 1.0 }
fn default_contrast()               -> f32          { 1.0 }
fn default_is_color()               -> bool         { false }
fn default_aspect_ratio_correction()-> f32          { 0.55 }

// ===== Configuration Struct =====
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConverterConfig {
    /// Character set for ASCII representation (dark to light).
    #[serde(default = "default_charset")]
    pub character_set: Vec<char>,

    /// Width of the output ASCII art (in characters).
    #[serde(default = "default_output_width")]
    pub output_width: u32,

    /// Optional height of the output (in characters). If `None`, calculated from aspect ratio.
    #[serde(default = "default_output_height")]
    pub output_height: Option<u32>,

    /// Brightness adjustment factor (1.0 = no change).
    #[serde(default = "default_brightness")]
    pub brightness_factor: f32,

    /// Contrast adjustment factor (1.0 = no change).
    #[serde(default = "default_contrast")]
    pub contrast_factor: f32,

    /// Whether to include color in the output.
    #[serde(default = "default_is_color")]
    pub is_color: bool,

    /// Aspect ratio correction factor (default 0.55 for ASCII art).
    #[serde(default = "default_aspect_ratio_correction")]
    pub aspect_ratio_correction: f32,
}

// ===== Configuration Struct Tests =====
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_defaults() {
        let json = json!({ });
        let config: ConverterConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.character_set, default_charset());
        assert_eq!(config.output_width, default_output_width());
        assert_eq!(config.output_height, default_output_height());
        assert_eq!(config.brightness_factor, default_brightness());
        assert_eq!(config.contrast_factor, default_contrast());
        assert_eq!(config.is_color, default_is_color());
        assert_eq!(config.aspect_ratio_correction, default_aspect_ratio_correction());
    }

    #[test]
    fn test_partial_deserialize() {
        let json = json!({
            "output_width": 80,
            "is_color": true
        });
        let config: ConverterConfig = serde_json::from_value(json).unwrap();
        // Check that partial deserialization is correct
        assert_eq!(config.output_width, 80);
        assert_eq!(config.is_color, true);

        // Check that defaults are still set for other fields
        assert_eq!(config.character_set, default_charset());
        assert_eq!(config.output_height, default_output_height());
        assert_eq!(config.brightness_factor, default_brightness());
        assert_eq!(config.contrast_factor, default_contrast());
        assert_eq!(config.aspect_ratio_correction, default_aspect_ratio_correction());
    }

    #[test]
    fn test_full_deserialize() {
        let json = json!({
            "character_set": ["#", "."],
            "output_width": 100,
            "output_height": 50,
            "brightness_factor": 2.0,
            "contrast_factor": 0.5,
            "is_color": true,
            "aspect_ratio_correction": 1.0
        });
        let config: ConverterConfig = serde_json::from_value(json).unwrap();
        assert_eq!(config.character_set, vec!['#', '.']);
        assert_eq!(config.output_width, 100);
        assert_eq!(config.output_height, Some(50));
        assert_eq!(config.brightness_factor, 2.0);
        assert_eq!(config.contrast_factor, 0.5);
        assert_eq!(config.is_color, true);
        assert_eq!(config.aspect_ratio_correction, 1.0);
    }
}