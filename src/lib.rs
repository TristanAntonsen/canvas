use image::{Rgb, RgbImage};

pub type Color = [f64; 3];

pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![[0.0, 0.0, 0.0]; height]; width],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x][y] = color
    }

    pub fn save_png(&self, path: &str) {
        let width = self.pixels.len() as u32;
        let height = self.pixels[0].len() as u32;

        let mut img = RgbImage::new(width, height);
        let mut r;
        let mut g;
        let mut b;
        let mut color: [f64; 3];
        for x in 0..width {
            for y in 0..height {
                color = self.pixels[x as usize][y as usize];
                r = (color[0] * 255.0).round() as u8;
                g = (color[1] * 255.0).round() as u8;
                b = (color[2] * 255.0).round() as u8;

                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        println!("{} exported.", path);

        img.save(path).expect("Could not save png");
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
