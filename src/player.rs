
use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32
}

impl Player {
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
}
