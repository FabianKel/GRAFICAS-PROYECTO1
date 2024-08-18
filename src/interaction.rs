use crate::player::{self, Player};
use crate::framebuffer::Framebuffer;
use crate::caster::cast_ray;
use minifb::{Key, Window};

pub struct InteractibleBlock {
    x: f32,
    y: f32,
    action: Box<dyn Fn(&mut Player)>,
    key_e_pressed: bool,
}

impl InteractibleBlock {
    pub fn new(x: f32, y: f32, action: impl Fn(&mut Player) + 'static) -> Self {
        InteractibleBlock {
            x,
            y,
            action: Box::new(action),
            key_e_pressed: false,
        }
    }

    

        pub fn check_interaction(
            &mut self,
            framebuffer: &mut Framebuffer,
            maze: &Vec<Vec<char>>,
            player: &mut Player,
            block_size: usize,
            max_distance: f32,
            window: &Window,
        ) {
            let player_x = player.pos.x;
            let player_y = player.pos.y;
            let ray = cast_ray(framebuffer, maze, player, player.a, block_size, false);
            // Verifica si el jugador está lo suficientemente cerca del bloque para interactuar
            if ((self.x - player_x).abs() < block_size as f32 || (self.x - player_x).abs() > block_size as f32) &&
               ray.distance <= max_distance 
            {
                match ray.impact {
                    '(' | ')' | '[' | ']' => {
                        if window.is_key_down(Key::E) {
                            if !self.key_e_pressed {
                                (self.action)(player);
                                self.key_e_pressed = true;
                            }
                        } else {
                            self.key_e_pressed = false;
                        }
                    }
                    _ => {}
                }
            }
        }
}

pub fn handle_interaction(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    interactible_blocks: &mut [InteractibleBlock],
    block_size: usize,
    max_distance: f32,
    window: &Window,
) {
    for block in interactible_blocks.iter_mut() {
        block.check_interaction(framebuffer, maze, player, block_size, max_distance, window);
    }
}



// La función para teletransportar al jugador
pub fn teleport_player(player: &mut Player, block_name: String, new_x: f32, new_y: f32) {
    let player_pos = player.get_pos();
    println!("Teleported from block: '{block_name}' {player_pos} to {new_x},{new_y}");
    player.pos.x = new_x;
    player.pos.y = new_y;
    player.a += 180.0;
}