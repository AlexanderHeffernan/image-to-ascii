---
id: r5grdpdyqc0w2uaf41xp71z
title: Converter Module
desc: ''
updated: 1752050889596
created: 1752050289641
---

# Converter Module

The converter module handles the core image processing and ASCII conversion logic. It's structured as a modular system with clear separation of concerns.

## Module Structure

```mermaid
graph TD
    A[Converter Module] --> B[core.rs]
    A --> C[config.rs]
    A --> D[ascii_pixel.rs]
    A --> E[error.rs]

    B --> G[Image Loading]
    B --> H[Color Adjustment]
    B --> I[ASCII Grid Generation]

    C --> J[Configuration Validation]
    C --> K[Default Values]

    D --> L[Pixel Representation]

    E --> M[Error Handling]
```

## Core Conversion Process

```mermaid
sequenceDiagram
    participant C as Client
    participant CV as Converter
    participant IMG as Image Processor
    participant GRID as Grid Builder

    C->>CV: convert_from_bytes(image_bytes, config)
    CV->>CV: validate_config(config)
    CV->>IMG: load_image(image_bytes)
    IMG-->>CV: Image object
    CV->>CV: calculate_output_height(image)

    alt Color Mode
        CV->>IMG: resize to RGB8
        CV->>GRID: build_ascii_grid (with color)
        loop For each pixel
            GRID->>CV: adjust_color()
            GRID->>CV: intensity_to_char()
            GRID->>GRID: create AsciiPixel with RGB
        end
    else Grayscale Mode
        CV->>IMG: resize to Luma8
        CV->>GRID: build_ascii_grid (no color)
        loop For each pixel
            GRID->>CV: intensity_to_char()
            GRID->>GRID: create AsciiPixel without RGB
        end
    end

    GRID-->>CV: ASCII Grid
    CV-->>C: Vec<Vec<AsciiPixel>>
```

## Key Functions

### `convert_from_bytes()`
Main entry point that orchestrates the entire conversion process.

1. **Validation** - Ensures configuration parameters are valid.
2. **Image Loading** - Loads image from memory using the `image` crate.
3. **Dimension Calculation** - Calculates output height if not specified.
4. **Processing Branch** - Chooses color or grayscale processing path based on configuration.
5. **Grid Generation** - Builds the final ASCII grid.