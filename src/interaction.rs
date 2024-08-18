// interaction.rs
use crate::player::Player;
use crate::framebuffer::Framebuffer;

pub struct InteractibleBlock {
    pub x: usize,
    pub y: usize,
    pub action: fn(&mut Player),
}

impl InteractibleBlock {
    pub fn new(x: usize, y: usize, action: fn(&mut Player)) -> Self {
        Self { x, y, action }
    }

    pub fn is_near(&self, player: &Player, block_size: usize, threshold: f32) -> bool {
        let player_x = player.pos.x / block_size as f32;
        let player_y = player.pos.y / block_size as f32;
        
        let block_x = self.x as f32;
        let block_y = self.y as f32;
        
        let distance = ((player_x - block_x).powi(2) + (player_y - block_y).powi(2)).sqrt();
        distance < threshold
    }
}

pub fn handle_interaction(player: &mut Player, interactible_blocks: &Vec<InteractibleBlock>, block_size: usize) {
    let threshold = 1.5;

    for block in interactible_blocks {
        if block.is_near(player, block_size, threshold) {
            if player.is_key_pressed("E") {
                (block.action)(player);
            }
        }
    }
}

pub fn teleport_player(player: &mut Player, new_x: f32, new_y: f32) {
    player.pos.x = new_x;
    player.pos.y = new_y;
}
