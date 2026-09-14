

use crate::projectile::Projectile;
use crate::player::Player;
use macroquad::prelude::*;

pub struct Projectiles {

    projectiles: Vec<Projectile>,
    space_was_pressed: bool,
    number_of_projectiles: usize


}

impl Projectiles {

    pub fn new() -> Self {
        Self {
            projectiles: vec![],
            space_was_pressed: false,
            number_of_projectiles: 3
        }
    }

    pub fn fire(&mut self, player : &Player) {
        self.projectiles.push(player.fire());
    }

    pub fn fly(&mut self) {
        for projectile in &mut self.projectiles {
            projectile.fly();
        }
    }

    pub fn draw(&self) {
        for projectile in &self.projectiles {
            projectile.draw();
        }
    }

    pub fn clean(&mut self) {

        let mut pops: Vec<usize> = vec![];
        let length   = self.projectiles.len();

        for i in 0..length {
            if self.projectiles[i].is_outside_window() {
                pops.push(i);
            }
        }

        for i in pops.iter().rev() {
            self.projectiles.remove(*i);
        }

    }

    pub fn update(&mut self, player : &Player) {
        let space_pressed = is_key_down(KeyCode::Space);

        if space_pressed && !self.space_was_pressed && self.projectiles.len() < self.number_of_projectiles {
        //if space_pressed && !self.space_was_pressed {
            self.fire(&player);
        }

        self.space_was_pressed = space_pressed;
    }

    pub fn check_collision(&mut self, x_start : f32, x_end : f32, y_start : f32, y_end : f32) -> bool{
        let length   = self.projectiles.len();

        for i in 0..length {
            if self.projectiles[i].check_collision(x_start, x_end, y_start, y_end) {
                self.projectiles.remove(i);
                return true;
            }
        }

        return false;
    }


}