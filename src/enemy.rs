

use macroquad::prelude::*;

pub struct Enemy {

    position : [f32; 2],
    velocity : f32

}

impl Enemy {

    pub fn new(x: f32, y: f32, v: f32) -> Self {
        Self {
            position: [x, y],
            velocity: v
        }
    }

    pub fn update(&mut self, points : i32) {
        self.position[1] += self.velocity + points as f32 * 0.1;
    }

    pub fn draw(&self) {
        draw_rectangle(self.position[0] - 20.0, self.position[1], 30.0, 14.0, BLUE);
    }

    pub fn has_passed_player(&self) -> bool {
        if self.position[1] > 700.0 {
            return true;
        }
        return false;
    }

    pub fn get_bounds(&self) -> Vec<f32> {
        return vec![self.position[0] - 20.0, self.position[0] + 20.0, self.position[1] - 7.0, self.position[1] + 7.0];
    }

}