use crate::draw::draw_cell;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;
use crate::caster::cast_ray;

pub fn render2d_mini(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    mini_x: usize,
    mini_y: usize,
    mini_scale: f32,
) {
    let mini_map_width = (maze[0].len() as f32 * block_size as f32 * mini_scale) as usize;
    let mini_map_height = (maze.len() as f32 * block_size as f32 * mini_scale) as usize;

    framebuffer.set_current_color(0xCCCCCC);
    for y in mini_y..mini_y + mini_map_height {
        for x in mini_x..mini_x + mini_map_width {
            framebuffer.point(x, y);
        }
    }

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let x = mini_x + (col as f32 * block_size as f32 * mini_scale) as usize;
            let y = mini_y + (row as f32 * block_size as f32 * mini_scale) as usize;
            let size = (block_size as f32 * mini_scale) as usize;
            draw_cell(framebuffer, x, y, size, maze[row][col]);
        }
    }

    framebuffer.set_current_color(0xFF00FF);
    let player_size = (40.0 * mini_scale) as usize;
    let player_x = mini_x + (player.pos.x as f32 * mini_scale) as usize;
    let player_y = mini_y + (player.pos.y as f32 * mini_scale) as usize;
    framebuffer.player(player_x - player_size / 2, player_y - player_size / 2, player_size);
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_manager: &TextureManager,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0x141f20);
    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    framebuffer.set_current_color(0x3e515a);
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);
        let distance = if intersect.distance > 10.0 {
            intersect.distance
        } else {
            10.0
        };

        let wall_height = ((hh + block_size as f32) as f32 / distance) * block_size as f32;
        let y0 = (hh - (wall_height / 2.0)) as usize;
        let y1 = (hh + (wall_height / 2.0)) as usize;

        let texture_identifier = match intersect.impact {
            '|' => "|_0".to_string(),
            '+' => "+_1".to_string(),
            '-' => "-_2".to_string(),
            '<' => "<_3".to_string(),
            '>' => ">_4".to_string(),
            '*' => "*_5".to_string(),
            '(' => "(_6".to_string(),
            ')' => "(_6".to_string(),
            '[' => "(_6".to_string(),
            ']' => "(_6".to_string(),
            _ => continue,
        };

        if let Some(texture) = texture_manager.get_texture(&texture_identifier) {
            let texture_width = texture.width() as f32;
            let texture_height = texture.height() as f32;

            let texture_index = if intersect.side == 'V' {
                intersect.hit_y % block_size as f32
            } else {
                intersect.hit_x % block_size as f32
            };

            let x_offset = (texture_index / block_size as f32) * texture_width;

            for y in y0..y1 {
                let texture_y = ((y as f32 - y0 as f32) / wall_height * texture_height) as u32;
                let pixel = texture.get_pixel(x_offset as u32, texture_y);
                framebuffer.set_current_color(
                    ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32),
                );
                framebuffer.point(i, y);
            }
        }
    }

    let mini_map_scale = 0.2;
    let mini_map_x = 1200;
    let mini_map_y = 10;
    render2d_mini(framebuffer, player, maze, block_size, mini_map_x, mini_map_y, mini_map_scale);
}
