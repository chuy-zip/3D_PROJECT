mod caster;
mod framebuffer;
mod maze;
mod player;

use std::time::{Duration, Instant};
use crate::caster::{cast_ray, load_textures};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use image::{DynamicImage, GenericImageView, Rgba};
use minifb::{Key, Window, WindowOptions, Scale};
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
    let t = t.powf(0.4); // Puedes ajustar el exponente para obtener el efecto deseado.

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
    let num_rays = framebuffer.width;
    let (texture_plus, texture_minus, texture_pipe, texture_g) = load_textures();

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;
    let max_distance = 120.0; // Ajusta esto según tu necesidad

    // Dibujar el degradado en el techo y el piso
    for y in 0..hh as usize {
        let distance_ratio = y as f32 / hh;
        let ceiling_color = interpolate_color(0x402905, 0x000000, distance_ratio);
        framebuffer.set_current_color(ceiling_color);
        for i in 0..framebuffer.width {
            framebuffer.point(i, y);
        }

        let floor_color = interpolate_color(0x6b5428, 0x000000, distance_ratio);
        framebuffer.set_current_color(floor_color);
        for i in 0..framebuffer.width {
            framebuffer.point(i, framebuffer.height - y - 1); // Asegúrate de no exceder los límites
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 100.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        let texture = match intersect.impact {
            '+' => &texture_plus,
            '-' => &texture_minus,
            '|' => &texture_pipe,
            'g' => &texture_g,
            _ => continue,
        };

        for y in stake_top..stake_bottom {
            let tex_y = ((y as f32 - stake_top as f32) / stake_height) * texture.height() as f32;
            let color = texture.get_pixel(
                (intersect.tex_coord * texture.width() as f32) as u32,
                tex_y as u32,
            );
            
            // Calcular la opacidad en función de la distancia
            let opacity = (1.0 - (distance_to_wall / max_distance)).clamp(0.0, 1.0);
            let blended_color = blend_color_with_opacity(color, opacity);
            
            framebuffer.set_current_color(
                (blended_color[0] as u32) << 16 | (blended_color[1] as u32) << 8 | blended_color[2] as u32,
            );
            framebuffer.point(i, y);
        }
    }
}

fn blend_color_with_opacity(color: Rgba<u8>, opacity: f32) -> Rgba<u8> {
    // Color negro
    let black = Rgba([0, 0, 0, 0]);

    // Mezclar el color con el negro basado en la opacidad
    Rgba([
        ((color[0] as f32 * opacity + black[0] as f32 * (1.0 - opacity)) as u8),
        ((color[1] as f32 * opacity + black[1] as f32 * (1.0 - opacity)) as u8),
        ((color[2] as f32 * opacity + black[2] as f32 * (1.0 - opacity)) as u8),
        color[3], // Mantener el canal alfa del color original
    ])
}

fn render2d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    block_siz2d: usize, // Cambié el nombre del parámetro para reflejar el tamaño del bloque en 2D
) {
    let scale_factor = block_siz2d as f32 / block_size as f32;

    // Escalar la posición del jugador para la vista 2D
    let player_pos_2d = Vec2::new(
        player.pos.x * scale_factor,
        player.pos.y * scale_factor,
    );

    // Dibujar el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(
                framebuffer,
                col * block_siz2d,
                row * block_siz2d,
                block_siz2d,
                maze[row][col],
            );
        }
    }

    // Dibujar al jugador
    framebuffer.set_current_color(0xFFDDD);
    framebuffer.point(player_pos_2d.x as usize, player_pos_2d.y as usize);

    // Lanzar rayos
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        cast_ray(framebuffer, &maze, &player, a, block_siz2d, true);
    }
}


fn main() {
    let window_width = 1080;
    let window_height = 720;
    let framebuffer_width = 650;
    let framebuffer_height = 450;
    let maze = load_maze("./maze.txt");
    let block_siz2d = 10;
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
        WindowOptions {
            resize: false,
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    framebuffer.set_background_color(0xb69f66);

    let mut player = Player::new(
        Vec2::new(100.0, 100.0),
        std::f32::consts::PI / 3.0,
        std::f32::consts::PI / 3.0,
    );

    let mut mode = "3D";

    window.set_position(0, 0);
    window.set_cursor_visibility(true);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
        if window.is_key_down(Key::Y){
            player.mouse_control = !player.mouse_control;
        }

        player.process_events(&window, &maze, block_size);

        framebuffer.clear();

        if mode == "3D" {
            render3d(&mut framebuffer, &player, &maze, block_size);
            render2d(&mut framebuffer, &player, &maze, block_size, block_siz2d);
        }

        fps_counter += 1;
        if last_frame_time.elapsed() >= Duration::from_secs(1) {
            current_fps = fps_counter;
            fps_counter = 0;
            last_frame_time = std::time::Instant::now();
        }

        framebuffer.set_current_color(0xFFFFFF); // Establece el color blanco para el texto
        framebuffer.draw_text(600, 10, &format!("FPS: {}", current_fps)); // Dibuja los FPS

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
