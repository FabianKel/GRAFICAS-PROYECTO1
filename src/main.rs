mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod render2d;
mod render3d;
mod draw;
mod events;
mod interaction;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;

use crate::render2d::render2d;
use crate::render3d::render3d;
use crate::events::process_events;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::textures::TextureManager;
use crate::interaction::{InteractibleBlock, handle_interaction, teleport_player};

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
    let maze = load_maze("./maze.txt");
    let block_size = 50;

    // Inicializa los bloques interactivos
    let mut interactible_blocks = vec![];

    let mut open_parx = 0.0;
    let mut open_pary = 0.0;
    let mut close_parx = 0.0;
    let mut close_pary = 0.0;
    let mut open_corx = 0.0;
    let mut open_cory = 0.0;
    let mut close_corx = 0.0;
    let mut close_cory = 0.0;

    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            let block_x = (i * block_size) as f32;
            let block_y = (j * block_size) as f32;
            if cell == '('{
                open_parx = block_x;
                open_pary = block_y;
                println!("Bloque (");
            }
            if cell == ')'{
                close_parx = block_x;
                close_pary = block_y;
                println!("Bloque )");
            }
            if cell == '['{
                open_corx = block_x;
                open_cory = block_y;
                println!("Bloque [");
            }
            if cell == ']'{
                close_corx = block_x;
                close_cory = block_y;
                println!("Bloque ]");
            }
        }
    }

    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            let block_x = (i * block_size) as f32;
            let block_y = (j * block_size) as f32;
            if cell == '('{
                interactible_blocks.push(InteractibleBlock::new(
                    block_x,
                    block_y,
                    move |player| {
                        teleport_player(player, close_parx as f32 - 50.0, close_pary as f32)
                    },
                ));
                println!("Bloque (");
                println!("X:{block_x} Y: {block_y} NewX: {close_parx} NewY:{close_pary}");
            }
            if cell == ')'{
                interactible_blocks.push(InteractibleBlock::new(
                    block_x,
                    block_y,
                    move |player| {
                        teleport_player(player, open_parx as f32 - 50.0, open_pary as f32)
                    },
                ));
                println!("Bloque )");
                println!("X:{block_x} Y: {block_y} NewX: {open_parx} NewY: {open_pary}");
            }
            if cell == '['{
                interactible_blocks.push(InteractibleBlock::new(
                    block_x,
                    block_y,
                    move |player| {
                        teleport_player(player, close_corx as f32 - 50.0, close_cory as f32)
                    },
                ));
                println!("Bloque [");
                println!("X:{block_x} Y: {block_y} NewX: {close_corx} NewY: {close_cory}");
                }
            if cell == ']'{
                interactible_blocks.push(InteractibleBlock::new(
                    block_x,
                    block_y,
                    move |player| {
                        teleport_player(player, open_corx as f32 - 50.0, open_cory as f32);
                    },
                ));
                println!("Bloque ]");
                println!("X:{block_x} Y: {block_y} NewX: {open_corx} NewX: {open_cory}");
            }
        }
    }

    let mut previous_mouse_x = 0.0;
    if let Some((initial_mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        previous_mouse_x = initial_mouse_x;
    }

    while window.is_open() {
        framebuffer.clear();

        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        // Procesa eventos de movimiento y acciones
        process_events(&mut player1, &mut window, &maze, block_size, &mut previous_mouse_x);

        // Maneja la interacción con bloques especiales
        handle_interaction(
            &mut framebuffer,
            &maze,
            &mut player1,
            &mut interactible_blocks, // Pasar como mutable
            block_size,
            500.0,
            &window,
        );

        if mode == "2D" {
            render2d(&mut framebuffer, &player1);
        } else {
            render3d(&mut framebuffer, &player1, &texture_manager, &maze, block_size);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(close_delay);
    }
}
