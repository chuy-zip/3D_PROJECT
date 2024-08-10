mod caster;
mod framebuffer;
mod maze;
mod player;

use crate::caster::cast_ray;
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

    let hw = framebuffer.width as f32 / 2.0; // precalculated half width
    let hh = framebuffer.height as f32 / 2.0; // precalculated half height

    framebuffer.set_current_color(0xFFFFFF);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Calculate the height of the stake
        let distance_to_wall = intersect.distance; // how far is this wall from the player
        let distance_to_projection_plane = 100.0; // how far is the "player" from the "camera"
                                                  // this ratio doesn't really matter as long as it is a function of distance
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calculate the position to draw the stake
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Draw the stake directly in the framebuffer
        for y in stake_top..stake_bottom {
            framebuffer.point(i, y); // Assuming white color for the stake
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
        Vec2::new(150.0, 150.0),
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
