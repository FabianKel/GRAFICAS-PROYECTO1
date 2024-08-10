
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2; 
use std::f32::consts::PI;
use std::time::Duration;

mod framebuffer;
mod maze;
mod player;
mod caster;

use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo:usize, block_size: usize, cell: char) {
    if cell == ' ' {
      return;
    }

    framebuffer.set_current_color(0x000000);
    for x in xo..xo + block_size{
      for y in yo..yo + block_size {
        framebuffer.point(x, y);
      }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player){
  let maze = load_maze("./maze.txt");
  let block_size = 50;

  //maze
  for row in 0..maze.len() {
      for col in 0..maze[row].len() {
          draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
      }
  }

  //player
  framebuffer.set_current_color(0xFF00FF);

  framebuffer.point(player.pos.x as usize, player.pos.y as usize);

  //caster
  cast_ray(framebuffer, &maze, player, player.a, block_size);
  //FOV
  let num_rays = 5;
  for i in 0..num_rays {
      let current_ray = i as f32 / num_rays as f32;
      let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
      cast_ray(framebuffer, &maze, player, a, block_size);
  }
}

fn main() {
    
    let window_width = 50*31;
    let window_height = 50*17;
    let framebuffer_width = window_width;
    let framebuffer_height = window_height;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new("Backroom", window_width, window_height, WindowOptions::default()).unwrap();

    framebuffer.set_background_color(0x334157);
    let close_delay = Duration::from_millis(16);

    let player1 = Player {
      pos: Vec2::new(75.0, 75.0),
      a: PI/3.0,
      fov: PI/3.0
    };

    let mut mode = "2D";

    while window.is_open(){
      framebuffer.clear();

      if window.is_key_down(Key::Escape) {
        break;
      }
      if window.is_key_down(Key::M){
        mode = if mode == "2D" {"3D"} else {"2D"};
      }

      if mode == "2D" {
        render2d(&mut framebuffer,&player1);
      } else {
        render2d(&mut framebuffer,&player1);
      }

      window
        .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        .unwrap();

      std::thread::sleep(close_delay);
    }



}   
