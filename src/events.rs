use minifb::Key;
use minifb::Window;

use crate::player::Player;

pub fn process_events(
    player: &mut Player,
    window: &mut Window,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    previous_mouse_x: &mut f32,
) {
    let move_speed = 10.0;
    let collision_margin = 5.0;
    let rotate_speed = 0.2;

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
