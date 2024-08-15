use crate::framebuffer::Framebuffer;
use crate::player::Player;
use image::{DynamicImage, GenericImageView};

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tex_coord: f32, // Add this to track where on the wall the ray hit
}

pub fn load_textures() -> (DynamicImage, DynamicImage, DynamicImage, DynamicImage) {
    let texture_plus = image::open("./src/img/TECH_4E.PNG").unwrap();
    let texture_minus = image::open("./src/img/TECH_1E.PNG").unwrap();
    let texture_pipe = image::open("./src/img/TECH_3B.PNG").unwrap();
    let texture_g = image::open("./src/img/TECH_4F.PNG").unwrap();
    (texture_plus, texture_minus, texture_pipe, texture_g)
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
    is2d: bool,
) -> Intersect {
    let mut d = 0.0;
    framebuffer.set_current_color(0xFFFFFF);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        let x;
        let y;

        if is2d {
            x = (player.pos2d.x + cos) as usize;
            y = (player.pos2d.y + sin) as usize;
        } else {
            x = (player.pos.x + cos) as usize;
            y = (player.pos.y + sin) as usize;
        }

        let i = x / block_size;
        let j = y / block_size;

        // Asegúrate de que i y j están dentro de los límites del laberinto
        if i >= maze[0].len() || j >= maze.len() {
            break;
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        if maze[j][i] != ' ' && maze[j][i] != 's' && maze[j][i] != 'g' {
            let (hit_vertical, tex_coord) = if (x % block_size) == 0 {
                // Golpe vertical
                (true, (y % block_size) as f32 / block_size as f32)
            } else if (y % block_size) == 0 {
                // Golpe horizontal
                (false, (x % block_size) as f32 / block_size as f32)
            } else {
                // Determinar si es más cercano a un golpe vertical u horizontal
                let x_mod = (x % block_size) as f32 / block_size as f32;
                let y_mod = (y % block_size) as f32 / block_size as f32;
                if x_mod > y_mod {
                    (true, y_mod)
                } else {
                    (false, x_mod)
                }
            };

            return Intersect {
                distance: d,
                impact: maze[j][i],
                tex_coord,
            };
        }

        d += 1.0;
    }

    Intersect {
        distance: d,
        impact: ' ',
        tex_coord: 0.0,
    }
}
