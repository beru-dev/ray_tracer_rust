use std::process;
use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    canvas: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            canvas: vec![Color(0.0, 0.0, 0.0); width * height]
        }
    }

    fn coords_to_index(&self, x: usize, y: usize) -> Result<usize, String> {
        let index: usize = y * self.width + x;
        if index < self.canvas.len() {
            Ok(index)
        } else {
            Err(format!("Coordinates outside of canvas {} {} {}", x, y, index))
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let index = self.coords_to_index(x, y).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

        return self.canvas[index];
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.coords_to_index(x, y).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

        self.canvas[index] = color;
    }

    pub fn canvas_to_ppm(&self) -> String {
        let mut pixel_data = String::from("");
        let mut chars_on_line: usize = 0;
        for (i, pixel) in self.canvas.iter().enumerate() {
            let pixel_integers = pixel.integer_color_data();
            for channel in pixel_integers.iter() {
                let channel_string = channel.to_string();
                let channel_length = channel_string.len();
                let mut start_char = String::from(" ");
                if chars_on_line + channel_length > 70 {
                    start_char = String::from("\n");
                    chars_on_line = 0;
                } else if chars_on_line == 0 {
                    start_char = String::from("");
                }
                pixel_data = format!("{}{}{}", pixel_data, start_char, channel_string);
                chars_on_line += channel_length + 1;
            }
            if (i + 1) % self.width == 0 {
                pixel_data = format!("{}{}", pixel_data, "\n");
                chars_on_line = 0;
            }
        }
        format!("P3\n{} {}\n255\n{}", self.width, self.height, pixel_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.canvas[150], Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.canvas_to_ppm();
        let ppm_split: Vec<&str> = ppm.split("\n").collect();
        let expected = vec!["P3", "5 3", "255"];

        assert_eq!(ppm_split[0..3], expected);
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = c.canvas_to_ppm();
        let ppm_split: Vec<&str> = ppm.split("\n").collect();
        let expected = vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        ];

        assert_eq!(&ppm_split[3..6], expected);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for w in 0..c.width {
            for h in 0..c.height {
                c.write_pixel(w, h, Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.canvas_to_ppm();
        let ppm_split: Vec<&str> = ppm.split("\n").collect();
        let expected = vec![
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        ];

        assert_eq!(&ppm_split[3..7], expected);
    }

    #[test]
    fn ppm_files_terminate_in_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.canvas_to_ppm();
        let expected = "P3\n5 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n";

        assert_eq!(ppm, expected);
    }
}