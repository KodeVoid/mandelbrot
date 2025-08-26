use image::RgbImage;
use image::ImageBuffer;
use image::Rgb;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Copy)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real = (self.real * other.real) - (self.imaginary * other.imaginary);
        let imaginary = (self.imaginary * other.real) + (self.real * other.imaginary);
        Self { real, imaginary }
    }
}

impl Complex {
    fn abs(&self) -> f64 {
        let squared_real = self.real * self.real;
        let squared_imaginary = self.imaginary * self.imaginary;
        let total = squared_real + squared_imaginary;
        total.sqrt()
    }
}

fn calculate_mandelbrot(c: Complex,max_iter:u64) -> usize {
    let mut iterations: usize = 0;
    let mut z = Complex {
        real: 0.0,
        imaginary: 0.0,
    };
    
    for _i in 0..max_iter { // Fixed: removed * and = from loop syntax
        if z.abs() > 2.0 {
            break;
        }
        z = z * z + c;
        iterations += 1;
    }
    
    iterations
}

#[derive(Debug)]
enum PixelColor {
    Black,
    Purple,
    Indigo,
    Blue,
    Violet,
    Red,
    White
}
impl From<PixelColor> for Rgb<u8>{
    fn from( color:PixelColor)->Rgb<u8>{
        match color{
            PixelColor::Black=>Rgb([0,0,0]),
            PixelColor::Red=>Rgb([255,0,0]),
            PixelColor::Blue=>Rgb([0,0,255]),
            PixelColor::Violet=>Rgb([143,0,255]),
            PixelColor::Indigo=>Rgb([75,0,130]),
            PixelColor::White=>Rgb([255,255,255]),
            PixelColor::Purple => Rgb([128,0,128]),
        }
    }
}
fn pixel_to_complex(x: f64, y: f64,image_width:f64,image_height:f64) -> Complex {
    let image_width: f64 = image_width;
    let image_height: f64 = image_height;
    
    // Scale and center the coordinates to a meaningful range for Mandelbrot
    let scale = 10.0; // View range from -2 to 2
    let complex_x = (x - (image_width / 2.0)) * scale / image_width;
    let complex_y = (y - (image_height / 2.0)) * scale / image_height; 
    Complex {
        real: complex_x,
        imaginary: complex_y,
    }
}

fn mandelbrot_mapper(max_width:u64,max_height:u64,max_iter:u64)->HashMap<(usize,usize),usize>{
    let mut grid:HashMap<(usize,usize),usize> =HashMap::new();
    for i in 0..max_width{
        for j in 0..max_height{
            let complex = pixel_to_complex(i as f64,j as  f64,max_width as f64,max_height as f64);
            let escape_time=calculate_mandelbrot(complex,max_iter);
            grid.insert((i as usize, j as usize),escape_time);

        }
    }

    grid
}
fn map_escape_time_to_color(escape_time:usize)->PixelColor{
    match escape_time{
        0=>PixelColor::Black,
        1..=4=>PixelColor::Red,
        5..=6=>PixelColor::Indigo,
        7..=8=>PixelColor::Violet,
        9..=10=>PixelColor::Blue,
        _=>PixelColor::White
    }

}

fn render_to_png(){
    const WIDTH:u32=5000;
    const HEIGHT:u32=5000;
    let grid =mandelbrot_mapper(WIDTH as u64,HEIGHT as u64,1000);

    let mut img:RgbImage=ImageBuffer::new(WIDTH,HEIGHT);

    for pixel in img.pixels_mut() {
        *pixel=Rgb::from(PixelColor::Black)
    }

    for ((x,y),&escape_time) in grid.iter() {
        let color=map_escape_time_to_color(escape_time);
        img.put_pixel(*x as u32,*y as u32,Rgb::from(color))
    }
    let _ = img.save("img.png");
    
}

fn main() {
    render_to_png()
}