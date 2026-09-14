
use macroquad::prelude::*;
pub struct Projectile {

    position: [f32; 2],
    velocity: f32


}

impl Projectile {

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: [x, y],
            velocity: 10.0
        }
    }

    pub fn fly(&mut self) {
        self.position[1] -= self.velocity;
    }

    pub fn draw(&self) {
        draw_rectangle(self.position[0], self.position[1], 5.0, 5.0, RED);
    }

    pub fn is_outside_window(&self) -> bool {
        return self.position[1] < 0.0;
    }

    pub fn check_collision(&mut self, x_start : f32, x_end : f32, y_start : f32, y_end : f32) -> bool {
        //println!("{} {}", self.position[0], self.position[1]);

        if (self.position[0] >= x_start) &&
            (self.position[0] <= x_end) &&
            (self.position[1] >= y_start) &&
            (self.position[1] <= y_end){
            return true
        }

        return false
    }

}