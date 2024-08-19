mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod render2d;
mod render3d;
mod draw;
mod events;

use events::overlay;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Instant, Duration};

use crate::render2d::render2d;
use crate::render3d::render3d;
use crate::events::process_events;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::textures::TextureManager;

fn main() {
    let window_width = 50 * 31;
    let window_height = 50 * 17;
    let framebuffer_width = window_width;
    let framebuffer_height = window_height;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new("Backroom", window_width, window_height, WindowOptions::default())
        .unwrap();

    framebuffer.set_background_color(0x334157);
    let close_delay = Duration::from_millis(16);

    let mut player1 = Player {
        pos: Vec2::new(1450.0, 650.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let mut texture_manager = TextureManager::new();
    texture_manager.load_sprite_sheet("./src/assets/walls4.png", 50, 50);

    let mut mode = "2D";
    let (maze, special_positions) = load_maze("./maze.txt");
    let block_size = 50;

    // Inicializa los bloques interactivos
    

    let mut previous_mouse_x = 0.0;
    if let Some((initial_mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        previous_mouse_x = initial_mouse_x;
    }

    let mut last_time = Instant::now();

    while window.is_open() {
        framebuffer.clear();

        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;
        let  fps = (1.0 / elapsed) as u32;

        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        // Procesa eventos de movimiento y acciones
        process_events(&mut player1, &mut window, &maze, block_size, &mut previous_mouse_x, &special_positions, &mut framebuffer);

        if mode == "2D" {
            render2d(&mut framebuffer, &player1);
        } else {
            render3d(&mut framebuffer, &player1, &texture_manager, &maze, block_size);
        }

        overlay(&mut player1, &maze, block_size, &mut framebuffer);

        framebuffer.set_current_color(0xFFFFFF);
        framebuffer.draw_text("FPS", 50, 10, 3);
        framebuffer.display_fps(fps, 10, 10, 3);
        framebuffer.draw_text("PRESS P TO CHANGE MUSIC", 10, 810, 3);



        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(close_delay);
    }
}
