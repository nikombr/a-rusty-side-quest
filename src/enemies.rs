

use macroquad::prelude::*;
use crate::enemy::Enemy;
use crate::projectiles::Projectiles;

pub struct Enemies {

    enemies: Vec<Enemy>,
    time: i32

}

impl Enemies {

    pub fn new() -> Self {
        Self {
            enemies: vec![],
            time: 0
        }
    }

    pub fn spawn(&mut self) {

        if self.time % 100 == 0 {

            let n = rand::gen_range(40.0, 980.0);

            self.enemies.push(Enemy::new(n, 0.0, 1.0));

        }

    }

    pub fn update(&mut self, points : i32) {
        for enemy in &mut self.enemies {
            enemy.update(points);
        }
    }

    pub fn draw(&self) {
        for enemy in &self.enemies {
            enemy.draw();
        }
    }

    pub fn has_lost(&self) -> bool {
        for enemy in &self.enemies {
            if enemy.has_passed_player() {
                return true;
            }
        }
        return false;
    }

    pub fn check_projectile_collision(&mut self, projectiles : &mut Projectiles) -> bool {

        // Incement time
        self.time += 1;

        // Check for projectile collision
        let length  = self.enemies.len();

        for i in 0..length {
            let bounds: Vec<f32> = self.enemies[i].get_bounds();
            //println!("{} {} {} {}", bounds[0], bounds[1], bounds[2], bounds[3]);
            if projectiles.check_collision(bounds[0], bounds[1], bounds[2], bounds[3]) {
                self.enemies.remove(i);
                return true;
            }
        }

        return false;

    }

    pub fn clear(&mut self) {
        self.enemies.clear();
        self.time = 0;
    }



}