use crate::framebuffer::Framebuffer;

pub fn draw_cell(
    framebuffer: &mut Framebuffer,
    x: usize,
    y: usize,
    block_size: usize,
    wall_type: char,
) {
    let color = match wall_type {
        '|' => 0x8093b8,
        '+' => 0x89a5f7,
        '-' => 0x6076b1,
        '<' => 0x00ff00,
        '>' => 0x00ff00,
        '(' => 0xffffff,
        ')' => 0xffffff,
        '[' => 0xffffff,
        ']' => 0xffffff,

        _ => 0x000000,
    };
    framebuffer.set_current_color(color);

    for i in 0..block_size {
        for j in 0..block_size {
            framebuffer.point(x + i, y + j);
        }
    }
}
