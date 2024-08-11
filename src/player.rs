use nalgebra_glm::Vec2;
use minifb::{ Window, Key };

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // ángulo de vista
    pub fov: f32, // campo de vista
    pub move_speed: f32,
    pub run_multiplier: f32,
    pub last_mouse_x: Option<f32>,
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Player {
            pos,
            a,
            fov,
            move_speed: 5.0, // valor por defecto
            run_multiplier: 3.0, // valor por defecto para la velocidad al correr
            last_mouse_x: None, // Inicializa como None
        }
    }

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 17.0;

        // Manejo del mouse para rotación
        if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            let mouse_x = mouse_pos.0 as f32;
            if let Some(last_x) = self.last_mouse_x {
                let delta_x = mouse_x - last_x;
                self.a += delta_x * 0.005; // Ajusta la sensibilidad del mouse aquí
            }
            self.last_mouse_x = Some(mouse_x);
        }

        // Rotación con teclas A y D (mantén esta funcionalidad)
        if window.is_key_down(Key::A) {
            self.a -= ROTATION_SPEED;
        }

        if window.is_key_down(Key::D) {
            self.a += ROTATION_SPEED;
        }

        // Determinar la velocidad
        let speed = if window.is_key_down(Key::LeftShift) {
            self.move_speed * self.run_multiplier
        } else {
            self.move_speed
        };

        // Calcular la nueva posición en función de la dirección actual y la velocidad
        let new_x = self.pos.x + self.a.cos() * speed;
        let new_y = self.pos.y + self.a.sin() * speed;

        // Calcular los índices de celda para la nueva posición
        let new_i = (new_x / block_size as f32) as usize;
        let new_j = (new_y / block_size as f32) as usize;

        // Comprobar colisiones con paredes
        if window.is_key_down(Key::W) {
            if maze[new_j][new_i] != ' ' {
                return; // Si hay una pared, no actualizar la posición
            }
            self.pos.x = new_x;
            self.pos.y = new_y;
        }

        // Comprobar si se mueve hacia atrás
        if window.is_key_down(Key::S) {
            let new_x = self.pos.x - self.a.cos() * speed;
            let new_y = self.pos.y - self.a.sin() * speed;
            let new_i = (new_x / block_size as f32) as usize;
            let new_j = (new_y / block_size as f32) as usize;
            if maze[new_j][new_i] != ' ' {
                return;
            }
            self.pos.x = new_x;
            self.pos.y = new_y;
        }
    }
}
