use image::{DynamicImage, GenericImageView};
use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tex_coord: f32, // Add this to track where on the wall the ray hit
}

pub fn load_textures() -> (DynamicImage, DynamicImage, DynamicImage, DynamicImage) {
    let texture_plus = image::open("./src/img/BRICK_1A.PNG").unwrap();
    let texture_minus = image::open("./src/img/BRICK_1A.PNG").unwrap();
    let texture_pipe = image::open("./src/img/BRICK_1A.PNG").unwrap();
    let texture_g = image::open("./src/img/BRICK_1A.PNG").unwrap();
    (texture_plus, texture_minus, texture_pipe, texture_g)
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

    framebuffer.set_current_color(0xFFFFFF);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;
    
        if maze[j][i] == '+' {
            framebuffer.set_current_color(0x011f4b);
        }
    
        if maze[j][i] == '-' {
            framebuffer.set_current_color(0x005b96);
        }
    
        if maze[j][i] == '|' {
            framebuffer.set_current_color(0xb3cde0);
        }
    
        if maze[j][i] == 'g' {
            framebuffer.set_current_color(0xffffff);
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        if maze[j][i] != ' ' {
            let tex_coord = if a.cos().abs() > a.sin().abs() {
                (x % block_size) as f32 / block_size as f32
            } else {
                (y % block_size) as f32 / block_size as f32
            };
            return Intersect {
                distance: d,
                impact: maze[j][i],
                tex_coord,
            };
        }

        d += 10.0;
    }
}
