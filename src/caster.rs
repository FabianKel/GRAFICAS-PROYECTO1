
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub side: char,
    pub hit_x: f32,
    pub hit_y: f32,
}


pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(0xffffff);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;

        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        if maze[j][i] != ' ' {
            let side = if (x as usize % block_size) == 0 || (y as usize % block_size) == 0 {
                'V'
            } else {
                'H'
            };

            return Intersect {
                distance: d,
                impact: maze[j][i],
                side,
                hit_x: x,
                hit_y: y,
            };
        }

        if draw_line {
            framebuffer.point(x as usize, y as usize);
        }

        d += 1.0;
    }
}
