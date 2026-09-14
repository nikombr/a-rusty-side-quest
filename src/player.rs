

use macroquad::prelude::*;

use crate::projectile::Projectile;

pub struct Player {

    position : [f32; 2],
    velocity : f32
    
}

impl Player {

    pub fn new(x: f32, y: f32, v: f32) -> Self {
        Self {
            position: [x, y],
            velocity: v
        }
    }

    pub fn move_right(&mut self) {
        self.position[0] += self.velocity;
        if self.position[0] > 1024.0 {
            self.position[0] = 0.0;
        }
    }

    pub fn move_left(&mut self) {
        self.position[0] -= self.velocity;
        if self.position[0] < 0.0 {
            self.position[0] = 1024.0;
        }
    }

    pub fn update(&mut self) {

        if is_key_down(KeyCode::Left) {
            // move player
            self.move_left();
        }

        if is_key_down(KeyCode::Right) {
            // move player
            self.move_right();
        }
        
    }

    pub fn draw(&self) {
        draw_rectangle(self.position[0] - 20.0, self.position[1], 40.0, 20.0, GREEN);
    }

    pub fn fire(&self) -> Projectile {
        return Projectile::new(self.position[0], self.position[1]);
    }

}