use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;

use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::textures::TextureManager;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(0x000000);
    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;

    // Dibuja el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(
                framebuffer,
                col * block_size,
                row * block_size,
                block_size,
                maze[row][col],
            );
        }
    }

    // Dibuja el jugador
    framebuffer.set_current_color(0xFF00FF);

    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    // Caster
    cast_ray(framebuffer, &maze, player, player.a, block_size, false);

    // Campo de visión
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, player, a, block_size, false);
    }
}

fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_manager: &TextureManager,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    let num_rays = framebuffer.width;
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Dibuja el cielo
    framebuffer.set_current_color(0x87CEEB);
    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    // Dibuja el suelo con un color plano
    framebuffer.set_current_color(0x6B8E23); // Color del suelo
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    for i in 0..num_rays {
      let current_ray = i as f32 / num_rays as f32;
      let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
  
      let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);
      let distance = intersect.distance;
      let wall_height = (block_size as f32 / distance) * 200.0;
      let y0 = hh - (wall_height / 2.0);
      let y1 = hh + (wall_height / 2.0);
  
      // Obtén la textura correspondiente
      let texture_identifier = match intersect.impact {
          '|' => "|_0".to_string(),
          '+' => "+_1".to_string(),
          '-' => "-_2".to_string(),
          '.' => "._3".to_string(),
          _ => continue,
      };
  
      if let Some(texture) = texture_manager.get_texture(&texture_identifier) {
          let texture_width = texture.width() as f32;
          let texture_height = texture.height() as f32;
  
          // Calcula el offset horizontal en la textura
          let cos = distance * a.cos();
          let x_offset = cos.fract() * texture_width;
  
          // Dibuja la textura verticalmente
          for y in y0 as usize..y1 as usize {
              let texture_y = ((y as f32 - y0) / wall_height * texture_height) as u32;
              let pixel = texture.get_pixel(x_offset as u32, texture_y);
              framebuffer.set_current_color(
                  ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32),
              );
              framebuffer.point(i, y);
          }
      }
  }
  
}

fn process_events(player: &mut Player, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
  let move_speed = 10.0;
  let rotate_speed = PI / 10.0;
  let collision_margin = 5.0;

  // Calcula la nueva posición propuesta
  let new_x = player.pos.x + player.a.cos() * move_speed;
  let new_y = player.pos.y + player.a.sin() * move_speed;

  // Verifica la colisión en la dirección hacia adelante
  if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
      let x = (new_x + collision_margin * player.a.cos()) as usize;
      let y = (new_y + collision_margin * player.a.sin()) as usize;

      if maze[y / block_size][x / block_size] == ' ' {
          player.pos.x = new_x;
          player.pos.y = new_y;
      }
  }

  // Calcula la nueva posición propuesta hacia atrás
  let new_x = player.pos.x - player.a.cos() * move_speed;
  let new_y = player.pos.y - player.a.sin() * move_speed;

  // Verifica la colisión en la dirección hacia atrás
  if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
      let x = (new_x - collision_margin * player.a.cos()) as usize;
      let y = (new_y - collision_margin * player.a.sin()) as usize;

      if maze[y / block_size][x / block_size] == ' ' {
          player.pos.x = new_x;
          player.pos.y = new_y;
      }
  }

  // Rotación del jugador
  if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
      player.a -= rotate_speed;
  }

  if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
      player.a += rotate_speed;
  }
}

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
        pos: Vec2::new(75.0, 75.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    // Configura el TextureManager
    let mut texture_manager = TextureManager::new();
    texture_manager.load_sprite_sheet("./src/assets/walls.png", 50, 50);

    let mut mode = "2D";
    let maze = load_maze("./maze.txt");
    let block_size = 50;

    while window.is_open() {
        framebuffer.clear();

        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(&mut player1, &window, &maze, block_size );

        if mode == "2D" {
            render2d(&mut framebuffer, &player1);
        } else {
            let maze = load_maze("./maze.txt");
            let block_size = 50;
            render3d(&mut framebuffer, &player1, &texture_manager, &maze, block_size);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(close_delay);
    }
}
