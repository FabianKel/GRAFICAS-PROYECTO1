use crate::draw::draw_cell;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;

pub fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;

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

    framebuffer.set_current_color(0xFF00FF);

    let player_size = 14;
    framebuffer.player(
        player.pos.x as usize - player_size / 2,
        player.pos.y as usize - player_size / 2,
        player_size,
    );

    cast_ray(framebuffer, &maze, player, player.a, block_size, false);

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
}
