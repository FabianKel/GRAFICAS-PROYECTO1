use image::{RgbaImage, ImageReader};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct TextureManager {
    textures: HashMap<String, RgbaImage>, // Cambia a HashMap<String, RgbaImage>
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    // Carga una hoja de sprites y asigna texturas a todos los tipos de paredes
    pub fn load_sprite_sheet(&mut self, filename: &str, sprite_width: u32, sprite_height: u32) {
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);
        let img = ImageReader::new(reader)
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();
        let img_width = img.width();
        let img_height = img.height();

        let num_sprites_x = img_width / sprite_width;
        let num_sprites_y = img_height / sprite_height;

        let mut index = 0; // Utiliza un índice para asignar texturas a tipos de paredes
        let mut texture_map = HashMap::new();

        for y in 0..num_sprites_y {
            for x in 0..num_sprites_x {
                let left = x * sprite_width;
                let top = y * sprite_height;
                let mut sprite = RgbaImage::new(sprite_width, sprite_height);
                
                for sy in 0..sprite_height {
                    for sx in 0..sprite_width {
                        let px = img.get_pixel(left + sx, top + sy).clone(); // Clona el pixel para obtener el valor
                        sprite.put_pixel(sx, sy, px); // Usa el valor clonado
                    }
                }

                // Asocia la textura a un tipo de pared basado en el índice
                let wall_type = match index {
                    0 => "|_0".to_string(),
                    1 => "+_1".to_string(),
                    2 => "-_2".to_string(),
                    3 => "<_3".to_string(),
                    4 => ">_4".to_string(),
                    5 => "*_5".to_string(),
                    6 => "(_6".to_string(),
                    7 => "[_7".to_string(),
                    _ => continue,
                };

                texture_map.insert(wall_type, sprite);
                index += 1;
            }
        }

        self.textures = texture_map;
    }

    // Cambia el tipo de identificador a String
    pub fn get_texture(&self, identifier: &str) -> Option<&RgbaImage> {
        self.textures.get(identifier)
    }
}