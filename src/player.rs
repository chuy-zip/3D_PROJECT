use nalgebra_glm::Vec2;
use minifb::{ Window, Key };

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // ángulo de vista
    pub fov: f32, // campo de vista
    pub move_speed: f32,
    pub run_multiplier: f32,
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Player {
            pos,
            a,
            fov,
            move_speed: 5.0, // valor por defecto
            run_multiplier: 3.0, // valor por defecto para la velocidad al correr
        }
    }

    pub fn process_events(&mut self, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 17.0;

        // Rotate left
        if window.is_key_down(Key::A) {
            self.a -= ROTATION_SPEED;
        }
        
        // Rotate right
        if window.is_key_down(Key::D) {
            self.a += ROTATION_SPEED;
        }
        
        // Determine speed
        let speed = if window.is_key_down(Key::LeftShift) {
            self.move_speed * self.run_multiplier
        } else {
            self.move_speed
        };

        // Calculate the new position based on current direction and speed
        let new_x = self.pos.x + self.a.cos() * speed;
        let new_y = self.pos.y + self.a.sin() * speed;

        // Calculate the cell indices for the new position
        let new_i = (new_x / block_size as f32) as usize;
        let new_j = (new_y / block_size as f32) as usize;

        // Check for collisions with walls
        if window.is_key_down(Key::W) {
            if maze[new_j][new_i] != ' ' {
                // If there's a wall, don't update the position
                return;
            }
            self.pos.x = new_x;
            self.pos.y = new_y;
        }

        // Check for moving backward
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
