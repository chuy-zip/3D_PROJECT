use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::BufReader;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use nalgebra_glm::Vec2;
use minifb::{Window, Key};

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // ángulo de vista
    pub fov: f32, // campo de vista
    pub move_speed: f32,
    pub run_multiplier: f32,
    pub mouse_control: bool,
    pub sound_sink: Option<Arc<Mutex<Sink>>>, // Control de sonido
    pub _stream: Option<Arc<OutputStream>>,  // Mantenemos el OutputStream vivo
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Player {
            pos,
            a,
            fov,
            move_speed: 3.0, // valor por defecto
            run_multiplier: 2.0, // valor por defecto para la velocidad al correr
            mouse_control: false, // Inicializa como None
            sound_sink: None, // Inicialmente no hay sonido
            _stream: None, // Inicialmente no hay OutputStream
        }
    }

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 70.0; // Velocidad de rotación

        // Solo procesar la rotación con el mouse si mouse_control es verdadero
        if self.mouse_control {
            if let Some((mouse_x, _mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                let window_width = 1080.0;
                let center_zone_left = window_width * 0.4;
                let center_zone_right = window_width * 0.6;
                let rotation_speed = 0.05;

                if mouse_x < center_zone_left {
                    self.a -= rotation_speed;
                } else if mouse_x > center_zone_right {
                    self.a += rotation_speed;
                }
            }
        }

        // Manejo de teclas A y D para rotación adicional
        if window.is_key_down(Key::A) {
            self.a -= ROTATION_SPEED;
        }

        if window.is_key_down(Key::D) {
            self.a += ROTATION_SPEED;
        }

        let speed = if window.is_key_down(Key::LeftShift) {
            self.move_speed * self.run_multiplier
        } else {
            self.move_speed
        };

        let mut moved = false;

        // Calcular la nueva posición en función de la dirección actual y la velocidad
        let new_x = self.pos.x + self.a.cos() * speed;
        let new_y = self.pos.y + self.a.sin() * speed;
        let new_i = (new_x / block_size as f32) as usize;
        let new_j = (new_y / block_size as f32) as usize;

        if window.is_key_down(Key::W) {
            if maze[new_j][new_i] != ' ' && maze[new_j][new_i] != 's' && maze[new_j][new_i] != 'g' {
                return;
            }
            self.pos.x = new_x;
            self.pos.y = new_y;
            moved = true;
        }

        if window.is_key_down(Key::S) {
            let new_x = self.pos.x - self.a.cos() * speed;
            let new_y = self.pos.y - self.a.sin() * speed;
            let new_i = (new_x / block_size as f32) as usize;
            let new_j = (new_y / block_size as f32) as usize;
            if maze[new_j][new_i] != ' ' && maze[new_j][new_i] != 's' && maze[new_j][new_i] != 'g' {
                return;
            }
            self.pos.x = new_x;
            self.pos.y = new_y;
            moved = true;
        }

        if moved {
            // Si el jugador se mueve y no hay sonido en reproducción, inicia el sonido
            if self.sound_sink.is_none() {
                self.start_walking_sound();
            }
        } else {
            // Si el jugador deja de moverse y el sonido está en reproducción, detén el sonido
            if let Some(sink) = &self.sound_sink {
                if !sink.lock().unwrap().empty() {
                    sink.lock().unwrap().stop();
                    self.sound_sink = None;
                }
            }
        }
    }

    pub fn start_walking_sound(&mut self) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = File::open("./src/sound/steps.mp3").unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();

        sink.append(source);

        // Mantenemos el OutputStream vivo usando Arc
        self._stream = Some(Arc::new(stream));
        self.sound_sink = Some(Arc::new(Mutex::new(sink)));
    }

    pub fn get_current_tile(&self, maze: &Vec<Vec<char>>, block_size: usize) -> Option<char> {
        let i = (self.pos.x / block_size as f32) as usize;
        let j = (self.pos.y / block_size as f32) as usize;

        if i < maze[0].len() && j < maze.len() {
            Some(maze[j][i])
        } else {
            None
        }
    }
}
