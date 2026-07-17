use std::path::Path;

pub struct RTWImage {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_pixel: usize,  // Number of bytes per pixel (3 for RGB, 4 for RGBA)
}

impl Default for RTWImage {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
            bytes_per_pixel: 3,
        }
    }
}

impl RTWImage {
    pub fn new(filename: &str) -> Self {
        let mut image = Self::default();

        if image.load(filename) {
            return image;
        }
        eprintln!("ERROR: Could not load image file '{}'", filename);
        image
    }

    pub fn load(&mut self, filename: &str) -> bool {
        let paths = [
            filename.to_string(),
            format!("images/{}", filename),
            format!("../images/{}", filename),
            format!("../../images/{}", filename),
            format!("../../../images/{}", filename),
            format!("../../../../images/{}", filename),
            format!("../../../../../images/{}", filename),
            format!("../../../../../../images/{}", filename),
        ];

        for path in paths {
            if Path::new(&path).exists() {
                if let Ok(img) = image::open(&path) {
                    let rgb = img.to_rgb8();

                    self.width = rgb.width() as usize;
                    self.height = rgb.height() as usize;

                    self.data = rgb.into_raw();

                    return true;
                }
            }
        }

        false
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> [u8; 3] {
        if self.data.is_empty() {
            return [255, 0, 255];
        }
        let x = Self::clamp(x, 0, self.width);
        let y = Self::clamp(y, 0, self.height);
        let index = y * self.width * self.bytes_per_pixel + x * self.bytes_per_pixel;
        [self.data[index], self.data[index + 1], self.data[index + 2]]
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        // Return the value clamped to the range [low, high)
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}
