
use minifb::Key;
use minifb::Window;
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::framebuffer::Framebuffer;
use crate::caster::cast_ray;

use crate::player::Player;

pub fn process_events(
    player: &mut Player,
    window: &mut Window,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    previous_mouse_x: &mut f32,
    special_positions: &HashMap<char, Vec<(usize, usize)>>,
    framebuffer: &mut Framebuffer,
) {
    let move_speed = 10.0;
    let collision_margin = 5.0;
    let rotate_speed = 0.2;

    let angle = player.a;

    // Realizar un cast de rayos para detectar intersección con un bloque
    let intersect = cast_ray(framebuffer, maze, player, angle, block_size, false);

    // Verificar si el jugador está mirando un bloque especial y si está a una distancia cercana
    if intersect.distance < 50.0 && (intersect.impact == '(' || intersect.impact == ')' || intersect.impact == '[' || intersect.impact == ']') {
        println!("Press E");

        // Si se presiona 'E', realizar el teletransporte
        if window.is_key_down(Key::E) {
            match intersect.impact {
                '(' => {
                    if let Some(target_positions) = special_positions.get(&')') {
                        if let Some(&(tx, ty)) = target_positions.get(0) {
                            player.pos.x = (tx * block_size) as f32 - 25.0;
                            player.pos.y = (ty * block_size) as f32 + 25.0;
                            player.a = 45.0;
                        }
                    }
                }
                ')' => {
                    if let Some(target_positions) = special_positions.get(&'(') {
                        if let Some(&(tx, ty)) = target_positions.get(0) {
                            player.pos.x = (tx * block_size) as f32 - 25.0;
                            player.pos.y = (ty * block_size) as f32 + 25.0;
                            player.a = 45.0;
                        }
                    }
                }
                '[' => {
                    if let Some(target_positions) = special_positions.get(&']') {
                        if let Some(&(tx, ty)) = target_positions.get(0) {
                            player.pos.x = (tx * block_size) as f32 - 25.0;
                            player.pos.y = (ty * block_size) as f32 + 25.0;
                            player.a = 45.0;
                        }
                    }
                }
                ']' => {
                    if let Some(target_positions) = special_positions.get(&'[') {
                        if let Some(&(tx, ty)) = target_positions.get(0) {
                            player.pos.x = (tx * block_size) as f32 - 25.0;
                            player.pos.y = (ty * block_size) as f32 + 25.0;
                            player.a = 45.0;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    // Calcula la nueva posición propuesta hacia adelante
    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        let new_x = player.pos.x + player.a.cos() * move_speed;
        let new_y = player.pos.y + player.a.sin() * move_speed;

        let x = (new_x + collision_margin * player.a.cos()) as usize;
        let y = (new_y + collision_margin * player.a.sin()) as usize;

        if maze[y / block_size][x / block_size] == ' ' {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    // Calcula la nueva posición propuesta hacia atrás
    if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        let new_x = player.pos.x - player.a.cos() * move_speed;
        let new_y = player.pos.y - player.a.sin() * move_speed;

        let x = (new_x - collision_margin * player.a.cos()) as usize;
        let y = (new_y - collision_margin * player.a.sin()) as usize;

        if maze[y / block_size][x / block_size] == ' ' {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    // Rotación del jugador basada en el movimiento del mouse
    if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        let mouse_delta_x = mouse_x - *previous_mouse_x;
        let mouse_sensitivity = 0.002;
        player.a += mouse_delta_x * mouse_sensitivity;
        *previous_mouse_x = mouse_x;
    }

    // Rotación del jugador con teclas
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        player.a -= rotate_speed;
    }

    if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        player.a += rotate_speed;
    }
}
