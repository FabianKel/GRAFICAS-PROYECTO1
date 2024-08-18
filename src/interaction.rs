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
        let ray = cast_ray(framebuffer, maze, player, player.a, block_size, false);

        if ray.distance <= max_distance {
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
    let mut teleported = false;

    for block in interactible_blocks.iter_mut() {
        if !teleported {
            let ray = cast_ray(framebuffer, maze, player, player.a, block_size, false);

            if ray.distance <= max_distance {
                if let '(' | ')' | '[' | ']' = ray.impact {
                    if window.is_key_down(Key::E) && !block.key_e_pressed {
                        (block.action)(player);
                        block.key_e_pressed = true;
                        teleported = true;  // Evita que otro bloque también teletransporte
                    } else if !window.is_key_down(Key::E) {
                        block.key_e_pressed = false;
                    }
                }
            }
        }
    }
}


// La función para teletransportar al jugador
pub fn teleport_player(player: &mut Player, new_x: f32, new_y: f32) {
    let player_pos = player.get_pos();
    println!("Teleported from {player_pos} to {new_x},{new_y}");
    player.pos.x = new_x;
    player.pos.y = new_y;
    player.a += 180.0;
}