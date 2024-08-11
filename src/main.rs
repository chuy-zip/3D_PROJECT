mod caster;
mod framebuffer;
mod maze;
mod player;

use std::time::{Duration, Instant};
use crate::caster::{cast_ray, load_textures};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use image::{DynamicImage, GenericImageView};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    if cell == '+' {
        framebuffer.set_current_color(0x011f4b);
    }

    if cell == '-' {
        framebuffer.set_current_color(0x005b96);
    }

    if cell == '|' {
        framebuffer.set_current_color(0xb3cde0);
    }

    if cell == 'g' {
        framebuffer.set_current_color(0xffffff);
    }

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.point(x, y);
        }
    }
}

fn interpolate_color(start: u32, end: u32, t: f32) -> u32 {
    // Usa un exponente mayor para un cambio más fuerte hacia el color final.
    let t = t.powf(0.3); // Puedes ajustar el exponente para obtener el efecto deseado.

    let sr = (start >> 16) & 0xFF;
    let sg = (start >> 8) & 0xFF;
    let sb = start & 0xFF;

    let er = (end >> 16) & 0xFF;
    let eg = (end >> 8) & 0xFF;
    let eb = end & 0xFF;

    let r = sr as f32 + (er as f32 - sr as f32) * t;
    let g = sg as f32 + (eg as f32 - sg as f32) * t;
    let b = sb as f32 + (eb as f32 - sb as f32) * t;

    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}



fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    let max_distance = 150.0; // distancia máxima que el jugador puede ver
    framebuffer.draw_floor_and_ceiling(0x402905, 0xb69f66);
    let num_rays = framebuffer.width;
    let (texture_plus, texture_minus, texture_pipe, texture_g) = load_textures();

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Dibujar el degradado en el techo y el piso primero
    for y in 0..hh as usize {
        let distance_ratio = y as f32 / hh;
        let ceiling_color = interpolate_color(0x402905, 0x000000, distance_ratio);
        framebuffer.set_current_color(ceiling_color);
        for i in 0..framebuffer.width {
            framebuffer.point(i, y);
        }

        let floor_color = interpolate_color(0xb69f66, 0x000000, distance_ratio);
        framebuffer.set_current_color(floor_color);
        for i in 0..framebuffer.width {
            framebuffer.point(i, framebuffer.height - y - 1);
        }
    }

    // Luego renderizar las paredes
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 100.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Si la distancia a la pared es mayor que la distancia máxima, usar color negro
        if distance_to_wall > max_distance {
            framebuffer.set_current_color(0x000000); // Color negro
        } else {
            let texture = match intersect.impact {
                '+' => &texture_plus,
                '-' => &texture_minus,
                '|' => &texture_pipe,
                'g' => &texture_g,
                _ => continue,
            };

            // Renderizar las texturas de las paredes
            for y in stake_top..stake_bottom {
                let tex_y = ((y as f32 - stake_top as f32) / stake_height) * texture.height() as f32;
                let color = texture.get_pixel(
                    (intersect.tex_coord * texture.width() as f32) as u32,
                    tex_y as u32,
                );
                framebuffer.set_current_color(
                    (color[0] as u32) << 16 | (color[1] as u32) << 8 | color[2] as u32,
                );
                framebuffer.point(i, y);
            }
            continue;
        }

        // Si se está usando el color negro, dibujar el stake completo en negro
        for y in stake_top..stake_bottom {
            framebuffer.point(i, y);
        }
    }
}



fn render2d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    // draws maze
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(
                framebuffer,
                col * block_size,
                row * block_size,
                block_size,
                maze[row][col],
            )
        }
    }

    // draw player
    framebuffer.set_current_color(0xFFDDD);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    // cast ray
    // cast_ray(framebuffer, &maze, &player, player.a, block_size);
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

fn main() {
    let window_width = 650;
    let window_height = 450;
    let framebuffer_width = 650;
    let framebuffer_height = 450;
    let maze = load_maze("./maze.txt");
    let block_size = 50;
    let frame_delay = Duration::from_millis(16);

    let mut last_frame_time = std::time::Instant::now();
    let mut fps_counter = 0;
    let mut current_fps = 0;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0xb69f66);

    let mut player = Player::new(
        Vec2::new(75.0, 75.0),
        std::f32::consts::PI / 3.0,
        std::f32::consts::PI / 3.0,
    );

    let mut mode = "2D";

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        player.process_events(&window, &maze, block_size);

        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze, block_size);
        } else {
            render3d(&mut framebuffer, &player, &maze, block_size)
        }

        fps_counter += 1;
        if last_frame_time.elapsed() >= Duration::from_secs(1) {
            current_fps = fps_counter;
            fps_counter = 0;
            last_frame_time = std::time::Instant::now();
        }

        framebuffer.set_current_color(0xFFFFFF); // Establece el color blanco para el texto
        framebuffer.draw_text(10, 10, &format!("FPS: {}", current_fps)); // Dibuja los FPS

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
