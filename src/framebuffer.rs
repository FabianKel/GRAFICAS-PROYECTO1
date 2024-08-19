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

    pub fn player(&mut self, x: usize, y: usize, size: usize) {
        for dx in 0..size {
            for dy in 0..size {
                let px = x + dx;
                let py = y + dy;
                if px < self.width && py < self.height {
                    self.point(px, py);
                }
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn draw_horizontal_line(&mut self, x: usize, y: usize, length: usize) {
        for i in 0..length {
            self.point(x + i, y);
        }
    }

    pub fn display_fps(&mut self, fps: u32, x: usize, y: usize, scale: usize) {
        let fps_text = fps.to_string();
        let mut cursor_x = x;

        for c in fps_text.chars() {
            if let Some(digit) = c.to_digit(10) {
                self.draw_digit(digit as usize, cursor_x, y, scale);
                cursor_x += 6 * scale; // Ajuste del espacio entre dígitos en función de la escala
            }
        }
    }

    fn draw_digit(&mut self, digit: usize, x: usize, y: usize, scale: usize) {
        let font = Self::FONT[digit];

        for (row, bits) in font.iter().enumerate() {
            for col in 0..5 {
                if (bits >> (4 - col)) & 1 == 1 {
                    self.draw_scaled_point(x + col * scale, y + row * scale, scale);
                }
            }
        }
    }

    fn draw_scaled_point(&mut self, x: usize, y: usize, scale: usize) {
        for dx in 0..scale {
            for dy in 0..scale {
                self.point(x + dx, y + dy);
            }
        }
    }

    const FONT: [[u8; 5]; 10] = [
        [0b11111, 0b10001, 0b10001, 0b10001, 0b11111], // 0
        [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
        [0b11111, 0b00001, 0b11111, 0b10000, 0b11111], // 2
        [0b11111, 0b00001, 0b11111, 0b00001, 0b11111], // 3
        [0b10001, 0b10001, 0b11111, 0b00001, 0b00001], // 4
        [0b11111, 0b10000, 0b11111, 0b00001, 0b11111], // 5
        [0b11111, 0b10000, 0b11111, 0b10001, 0b11111], // 6
        [0b11111, 0b00001, 0b00001, 0b00001, 0b00001], // 7
        [0b11111, 0b10001, 0b11111, 0b10001, 0b11111], // 8
        [0b11111, 0b10001, 0b11111, 0b00001, 0b11111], // 9
    ];

    // Define el mapa de bits para las letras A-Z (mayúsculas)
    const FONT_ALPHA: [[u8; 5]; 26] = [
        [0b01110, 0b10001, 0b11111, 0b10001, 0b10001], // A
        [0b11110, 0b10001, 0b11110, 0b10001, 0b11110], // B
        [0b01110, 0b10001, 0b10000, 0b10001, 0b01110], // C
        [0b11110, 0b10001, 0b10001, 0b10001, 0b11110], // D
        [0b11111, 0b10000, 0b11110, 0b10000, 0b11111], // E
        [0b11111, 0b10000, 0b11110, 0b10000, 0b10000], // F
        [0b01110, 0b10000, 0b10011, 0b10001, 0b01110], // G
        [0b10001, 0b10001, 0b11111, 0b10001, 0b10001], // H
        [0b01110, 0b00100, 0b00100, 0b00100, 0b01110], // I
        [0b00001, 0b00001, 0b00001, 0b10001, 0b01110], // J
        [0b10001, 0b10010, 0b11100, 0b10010, 0b10001], // K
        [0b10000, 0b10000, 0b10000, 0b10000, 0b11111], // L
        [0b10001, 0b11011, 0b10101, 0b10001, 0b10001], // M
        [0b10001, 0b11001, 0b10101, 0b10011, 0b10001], // N
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // O
        [0b11110, 0b10001, 0b11110, 0b10000, 0b10000], // P
        [0b01110, 0b10001, 0b10001, 0b01110, 0b00011], // Q
        [0b11110, 0b10001, 0b11110, 0b10010, 0b10001], // R
        [0b01111, 0b10000, 0b01110, 0b00001, 0b11110], // S
        [0b11111, 0b00100, 0b00100, 0b00100, 0b00100], // T
        [0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // U
        [0b10001, 0b10001, 0b10001, 0b01010, 0b00100], // V
        [0b10001, 0b10001, 0b10101, 0b11011, 0b10001], // W
        [0b10001, 0b01010, 0b00100, 0b01010, 0b10001], // X
        [0b10001, 0b01010, 0b00100, 0b00100, 0b00100], // Y
        [0b11111, 0b00010, 0b00100, 0b01000, 0b11111], // Z
        ];
    
    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, scale: usize) {
        let mut cursor_x = x;
        for c in text.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as usize;
                self.draw_digit(digit, cursor_x, y, scale);
                cursor_x += 6 * scale; // Espacio entre caracteres
            } else if c.is_alphabetic() {
                let letter = c.to_ascii_uppercase() as usize - 'A' as usize;
                self.draw_letter(letter, cursor_x, y, scale);
                cursor_x += 6 * scale; // Espacio entre caracteres
            } else if c == ' ' {
                    cursor_x += 4 * scale; // Espacio para un espacio
                }
            }
        }
    
    fn draw_letter(&mut self, letter: usize, x: usize, y: usize, scale: usize) {
        let font = Self::FONT_ALPHA[letter];
        for (row, bits) in font.iter().enumerate() {
            for col in 0..5 {
                if (bits >> (4 - col)) & 1 == 1 {
                   self.draw_scaled_point(x + col * scale, y + row * scale, scale);
                }
            }
        }
    }
        
}
