use crate::caster::{cast_ray, load_textures};
use crate::framebuffer::Framebuffer;
use crate::maze::{find_start_position, load_maze};
use crate::player::Player;
use crate::sfx::{play_background_music, play_sound};
use image::{DynamicImage, GenericImageView, Rgba};
use minifb::{Key, Scale, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::time::{Duration, Instant};

// Definición del enum para los estados del juego
pub enum GameState {
    WelcomeScreen,
    Playing,
    EndScreen, // Otros estados como MainMenu, GameOver, etc.
}

// Estructura principal que representa el juego
pub struct Game {
    pub window: Window,
    pub state: GameState,
    pub player: Player,
    pub framebuffer: Framebuffer,
    pub maze_opt: usize,
    pub maze: Vec<Vec<char>>,
    pub mode: &'static str,
    pub last_frame_time: Instant,
    pub fps_counter: usize,
    pub current_fps: usize,
    pub prev_y_pressed: bool,
    pub m_pressed: bool,
    pub frame_delay: Duration,
    pub block_size: usize,
    pub block_siz2d: usize,
}

impl Game {
    pub fn new() -> Self {
        let block_size = 30;
        let block_siz2d = 5;

        let window_width = 1080;
        let window_height = 720;
        let framebuffer_width = block_size * 13; //390
        let framebuffer_height = block_size * 9; //270
        let frame_delay = Duration::from_millis(20);

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

        window.set_position(0, 0);
        window.set_cursor_visibility(true);

        let maze = load_maze("./maze.txt");

        let mut player = Player::new(
            Vec2::new(30.0, 30.0),
            std::f32::consts::PI / 3.0,
            std::f32::consts::PI / 3.5,
        );

        if let Some((start_x, start_y)) = find_start_position(&maze, block_size) {
            player.pos = Vec2::new(start_x as f32, start_y as f32);
        } else {
            panic!("No start position ('s') found in the maze!");
        }

        let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
        framebuffer.set_background_color(0xb69f66);

        play_background_music("./src/sound/background.mp3");

        Game {
            window,
            state: GameState::WelcomeScreen,
            player,
            framebuffer,
            maze_opt: 1,
            maze,
            mode: "3D",
            last_frame_time: Instant::now(),
            fps_counter: 0,
            current_fps: 0,
            prev_y_pressed: false,
            m_pressed: false,
            frame_delay,
            block_size,
            block_siz2d,
        }
    }

    pub fn render(&mut self) {
        match self.state {
            GameState::Playing => self.render_playing(),
            GameState::WelcomeScreen => self.render_tittle_screen(),
            GameState::EndScreen => self.render_end_screen(), // Otros estados se manejarían aquí
        }
    }

    fn render_tittle_screen(&mut self) {
        self.framebuffer.clear();
        self.framebuffer
            .draw_image("./src/img/tittleScreen.png", 0, 0);

        self.window
            .update_with_buffer(
                &self.framebuffer.buffer,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .unwrap();

        if self.window.is_key_down(Key::Key1) {

            self.maze = load_maze("./maze.txt");

            if let Some((start_x, start_y)) = find_start_position(&self.maze, self.block_size) {
                self.player.pos = Vec2::new(start_x as f32, start_y as f32);
            } else {
                panic!("No start position ('s') found in the maze!");
            }

            self.maze_opt = 1;
            self.state = GameState::Playing;
        }

        if self.window.is_key_down(Key::Key2) {

            self.maze = load_maze("./maze2.txt");

            if let Some((start_x, start_y)) = find_start_position(&self.maze, self.block_size) {
                self.player.pos = Vec2::new(start_x as f32, start_y as f32);
            } else {
                panic!("No start position ('s') found in the maze!");
            }
            
            self.maze_opt = 2;
            self.state = GameState::Playing;
        }

        if self.window.is_key_down(Key::Key3) {

            self.maze = load_maze("./maze3.txt");

            if let Some((start_x, start_y)) = find_start_position(&self.maze, self.block_size) {
                self.player.pos = Vec2::new(start_x as f32, start_y as f32);
            } else {
                panic!("No start position ('s') found in the maze!");
            }

            self.maze_opt = 3;
            self.state = GameState::Playing;
        }

        if self.window.is_key_down(Key::Escape) {
            return;
        }
    }

    fn render_end_screen(&mut self) {
        self.framebuffer.clear();
        self.framebuffer.draw_image("./src/img/endScreen.png", 0, 0);

        self.window
            .update_with_buffer(
                &self.framebuffer.buffer,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .unwrap();

        if self.window.is_key_down(Key::Enter) {
            self.state = GameState::WelcomeScreen;
        }

        if self.window.is_key_down(Key::Escape) {
            return;
        }
    }

    fn render_playing(&mut self) {
        fn draw_cell(
            framebuffer: &mut Framebuffer,
            xo: usize,
            yo: usize,
            block_size: usize,
            cell: char,
        ) {
            if cell == ' ' || cell == 's' {
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
                framebuffer.set_current_color(0xffbf00);
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
                let ceiling_color = interpolate_color(0x252423, 0x000000, distance_ratio);
                framebuffer.set_current_color(ceiling_color);
                for i in 0..framebuffer.width {
                    framebuffer.point(i, y);
                }

                let floor_color = interpolate_color(0x5b6567, 0x000000, distance_ratio);
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
                let distance_to_projection_plane = 90.0;
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
                    let tex_y =
                        ((y as f32 - stake_top as f32) / stake_height) * texture.height() as f32;
                    let color = texture.get_pixel(
                        (intersect.tex_coord * texture.width() as f32) as u32,
                        tex_y as u32,
                    );

                    // Calcular la opacidad en función de la distancia
                    let opacity = (1.0 - (distance_to_wall / max_distance)).clamp(0.0, 1.0);
                    let blended_color = blend_color_with_opacity(color, opacity);

                    framebuffer.set_current_color(
                        (blended_color[0] as u32) << 16
                            | (blended_color[1] as u32) << 8
                            | blended_color[2] as u32,
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
            view: bool,
        ) {
            let scale_factor = block_siz2d as f32 / block_size as f32;

            // Escalar la posición del jugador para la vista 2D
            let player_pos_2d = Vec2::new(player.pos.x * scale_factor, player.pos.y * scale_factor);

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

                cast_ray(framebuffer, &maze, &player, a, block_siz2d, view);
            }
        }

        if self.window.is_key_down(Key::Escape) {
            return;
        }

        if self.window.is_key_down(Key::Enter) {
            self.state = GameState::Playing;
        }

        let selected_maze = match self.maze_opt {
            1 => "./maze.txt",
            2 => "./maze2.txt",
            3 => "./maze3.txt",
            _ => "./default_maze.txt", // Por si acaso hay un valor fuera del rango esperado
        };

        let current_tile = self.player.get_current_tile(&self.maze, self.block_size);

        if let Some('g') = current_tile {
            self.state = GameState::EndScreen;
        }

        if self.window.is_key_down(Key::M) {
            if !self.m_pressed {
                self.mode = if self.mode == "2D" { "3D" } else { "2D" };
                self.m_pressed = true;
                play_sound("./src/sound/digimap.mp3");
            }
        } else {
            self.m_pressed = false;
        }

        let is_y_pressed = self.window.is_key_down(Key::Y);

        if is_y_pressed && !self.prev_y_pressed {
            play_sound("./src/sound/digicam.mp3");
            self.player.mouse_control = !self.player.mouse_control;
        }

        self.prev_y_pressed = is_y_pressed;

        self.player
            .process_events(&self.window, &self.maze, self.block_size);

        self.framebuffer.clear();

        if self.mode == "3D" {
            render3d(
                &mut self.framebuffer,
                &self.player,
                &self.maze,
                self.block_size,
            );
            render2d(
                &mut self.framebuffer,
                &self.player,
                &self.maze,
                self.block_size,
                self.block_siz2d,
                false,
            );
        } else {
            render2d(
                &mut self.framebuffer,
                &self.player,
                &self.maze,
                self.block_size,
                self.block_size,
                true,
            );
        }

        self.window
            .update_with_buffer(
                &self.framebuffer.buffer,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .unwrap();

        self.fps_counter += 1;
        if self.last_frame_time.elapsed() >= Duration::from_secs(1) {
            self.current_fps = self.fps_counter;
            self.fps_counter = 0;
            self.last_frame_time = Instant::now();
        }

        self.framebuffer.set_current_color(0xFFFFFF); // Establece el color blanco para el texto
        self.framebuffer
            .draw_text(220, 10, &format!("FPS: {}", self.current_fps)); // Dibuja los FPS

        self.window
            .update_with_buffer(
                &self.framebuffer.buffer,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .unwrap();

        std::thread::sleep(self.frame_delay);
    }
    // Otros métodos según sea necesario
}
