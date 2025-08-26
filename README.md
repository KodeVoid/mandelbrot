# Mandelbrot Generator

## Overview
This is a program I wrote in Rust to generate the famous fractal known as the Mandelbrot set. The Mandelbrot set is a set of complex numbers that produces a fractal when plotted, characterized by its intricate, self-similar boundary.

## Features

- **Complex Number Implementation**:
  - I implemented a custom `Complex` struct to represent complex numbers with `real` and `imaginary` components.
  - Added `std::ops::{Mul, Add}` trait implementations to enable easy multiplication and addition of complex numbers.
  - Included an `abs()` method to calculate the distance from the origin (magnitude), facilitating the escape time algorithm.

- **calculate_mandelbrot()**:
  - This function implements the classical escape time algorithm to determine if a given complex number `c` exceeds a boundary (magnitude > 2.0) after a specified number of iterations.
  - The core equation is `z = z * z + c`, where `z` starts at `0 + 0i` and is iteratively updated.
  - Returns the number of iterations before the point escapes or the maximum iteration count if it remains bounded.

## How It Works
1. The program maps pixel coordinates to complex numbers using a custom transformation function.
2. For each pixel, `calculate_mandelbrot()` computes the escape time based on the Mandelbrot iteration.
3. The escape time is then mapped to a color and rendered as a PNG image.

## Thoughts
- I haven't quite gotten the pixel mapping to the complex plane and the color mapping correctly, so the generated fractal doesn't yet resemble the expected intricate shape. The current output lacks the detailed, self-similar boundary typical of the Mandelbrot set. Contributions and thoughts are welcome! Possible areas to explore include adjusting the `pixel_to_complex` or `pixel_to_point` function for accurate scaling and centering, or refining the `map_escape_time_to_color` function for a smoother gradient.

## Usage
1. Ensure the `image` crate is added to your `Cargo.toml`:
   ```toml
   [dependencies]
   image = "0.24"
