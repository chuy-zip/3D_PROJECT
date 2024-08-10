mod caster;
mod framebuffer;
mod maze;
mod player;

use image::{DynamicImage, GenericImageView};
use crate::caster::{cast_ray, load_textures};
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::time::Duration;

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

fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;
    let num_rays = framebuffer.width;
    let (texture_plus, texture_minus, texture_pipe, texture_g) = load_textures();

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

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

        // Texture sampling
        for y in stake_top..stake_bottom {
            let tex_y = ((y - stake_top) as f32 / stake_height) * texture.height() as f32;
            let color = texture.get_pixel(
                (intersect.tex_coord * texture.width() as f32) as u32,
                tex_y as u32,
            );
            framebuffer.set_current_color((color[0] as u32) << 16 | (color[1] as u32) << 8 | color[2] as u32);
            framebuffer.point(i, y);
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;

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
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x333355);

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

        player.process_events(&window);

        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, &player);
        } else {
            render3d(&mut framebuffer, &player)
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
