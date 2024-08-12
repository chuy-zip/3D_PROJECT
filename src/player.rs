use nalgebra_glm::Vec2;
use minifb::{Window, Key};

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // ángulo de vista
    pub fov: f32, // campo de vista
    pub move_speed: f32,
    pub run_multiplier: f32,
    pub mouse_control: bool,
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Player {
            pos,
            a,
            fov,
            move_speed: 5.0, // valor por defecto
            run_multiplier: 3.0, // valor por defecto para la velocidad al correr
            mouse_control: false, // Inicializa como None
        }
    }

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 70.0; // Velocidad de rotación
    
        // Solo procesar la rotación con el mouse si mouse_control es verdadero
        if self.mouse_control {
            if let Some((mouse_x, _mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                // Ancho de la ventana
                let window_width = 1080.0;
                
                // Definir los límites de las zonas
                let center_zone_left = window_width * 0.4;  // 40% desde la izquierda
                let center_zone_right = window_width * 0.6; // 60% desde la izquierda
        
                // Definir la velocidad de rotación constante
                let rotation_speed = 0.05;
        
                if mouse_x < center_zone_left {
                    // Zona izquierda: rotar hacia la izquierda
                    self.a -= rotation_speed;
                } else if mouse_x > center_zone_right {
                    // Zona derecha: rotar hacia la derecha
                    self.a += rotation_speed;
                }
                // Si el mouse está en la zona central, no se hace nada
            }
        }
    
        // Manejo de teclas A y D para rotación adicional
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
