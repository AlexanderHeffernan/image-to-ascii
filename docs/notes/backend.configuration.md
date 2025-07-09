---
id: 5cict2b9vh7agzzbgek95hp
title: Configuration System
desc: ''
updated: 1752051108852
created: 1752050921307
---

# Configuration System

The backend uses a flexible configuration system built with Serde for JSON serialization/deserialization with defaults.

## Configuration Structure

```mermaid
classDiagram
    class ConverterConfig {
        +Vec~char~ character_set
        +u32 output_width
        +Option~u32~ output_height
        +f32 brightness_factor
        +f32 contrast_factor
        +bool is_color
        +f32 aspect_ratio_correction
    }
    
    class DefaultFunctions {
        +default_charset() Vec~char~
        +default_output_width() u32
        +default_output_height() Option~u32~
        +default_brightness() f32
        +default_contrast() f32
        +default_is_color() bool
        +default_aspect_ratio_correction() f32
    }
    
    ConverterConfig --> DefaultFunctions : uses
```

## Example Configurations

### Minimal Configuration
```json
{
  "output_width": 100
}
```
All other values use defaults.

### Full Configuration
```json
{
  "character_set": [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"],
  "output_width": 200,
  "output_height": 100,
  "brightness_factor": 1.2,
  "contrast_factor": 0.8,
  "is_color": true,
  "aspect_ratio_correction": 0.55
}
```

### Color Configuration
```json
{
  "output_width": 150,
  "is_color": true,
  "brightness_factor": 1.1,
  "character_set": [" ", "█"]
}
```