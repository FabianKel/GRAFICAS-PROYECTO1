
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

mod framebuffer;
mod maze;

use framebuffer::Framebuffer;
use crate::maze::load_maze;

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

fn render(framebuffer: &mut Framebuffer){
  let maze = load_maze("./maze.txt");
  let block_size = 50;

  for row in 0..maze.len() {
      for col in 0..maze[row].len() {
          draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
      }
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
      framebuffer.clear();

      render(&mut framebuffer);

      window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();

      std::thread::sleep(close_delay);
    }



}   
