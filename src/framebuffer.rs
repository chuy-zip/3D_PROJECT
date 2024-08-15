use image::GenericImageView;

const FONT: [[u8; 5]; 10] = [
    [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // 0
    [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
    [0b01110, 0b10001, 0b00110, 0b01000, 0b11111], // 2
    [0b11110, 0b00101, 0b01110, 0b00101, 0b11110], // 3
    [0b00100, 0b01100, 0b10100, 0b11111, 0b00100], // 4
    [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], // 5
    [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], // 6
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000], // 7
    [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], // 8
    [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], // 9
];

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
    pub fn draw_floor_and_ceiling(&mut self, ceiling_color: u32, floor_color: u32) {
        // Color para el techo
        for y in 0..self.height / 2 {
            for x in 0..self.width {
                self.buffer[y * self.width + x] = ceiling_color;
            }
        }

        // Color para el piso
        for y in self.height / 2..self.height {
            for x in 0..self.width {
                self.buffer[y * self.width + x] = floor_color;
            }
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, ch: char) {
        if let Some(digit) = ch.to_digit(10) {
            let pattern = FONT[digit as usize];
            for (row, bits) in pattern.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        self.point(x + col, y + row);
                    }
                }
            }
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(x + i * 6, y, ch);
        }
    }

    pub fn draw_image(&mut self, path: &str, x_offset: usize, y_offset: usize) {
        // Carga la imagen desde el path especificado
        let img = image::open(path).unwrap();
        let img = img.to_rgba8(); // Asegúrate de que la imagen esté en formato RGBA
    
        let (img_width, img_height) = img.dimensions();
    
        // Recorre los píxeles de la imagen y cópialos en el framebuffer
        for y in 0..img_height {
            for x in 0..img_width {
                if x as usize + x_offset < self.width && y as usize + y_offset < self.height {
                    let pixel = img.get_pixel(x, y);
                    let alpha = pixel[3] as f32 / 255.0; // Componente Alfa (transparencia)
    
                    // Si el píxel no es completamente transparente
                    if alpha > 0.0 {
                        let background_color = self.buffer[(y as usize + y_offset) * self.width + (x as usize + x_offset)];
    
                        // Extrae los componentes RGB del color de fondo
                        let bg_r = ((background_color >> 16) & 0xFF) as f32;
                        let bg_g = ((background_color >> 8) & 0xFF) as f32;
                        let bg_b = (background_color & 0xFF) as f32;
    
                        // Mezcla los colores usando el valor alfa
                        let r = ((pixel[0] as f32 * alpha) + (bg_r * (1.0 - alpha))) as u32;
                        let g = ((pixel[1] as f32 * alpha) + (bg_g * (1.0 - alpha))) as u32;
                        let b = ((pixel[2] as f32 * alpha) + (bg_b * (1.0 - alpha))) as u32;
    
                        // Combina los componentes y asigna el color resultante al framebuffer
                        let rgba = (r << 16) | (g << 8) | b;
                        self.buffer[(y as usize + y_offset) * self.width + (x as usize + x_offset)] = rgba;
                    }
                }
            }
        }
    }
    
    
    
}
