
mod enemies;
mod enemy;
mod player;
mod projectile;
mod projectiles;

use player::Player;
use projectiles::Projectiles;
use enemies::Enemies;

use macroquad::prelude::*;



fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders: A Rusty Side Quest".to_string(),
        window_width: 1024,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    println!("Welcome to Space Invaders: A Rusty Side Quest!");
    let mut player = Player::new(500.0, 700.0, 10.0);
    let mut enemies: Enemies = Enemies::new();
    //let number_of_bullets: usize = 3;  
    //let mut projectiles: Vec<Projectile> = vec![];
    //let mut space_was_pressed = false;
    let mut projectiles = Projectiles::new();

    let mut has_lost = false;

    let mut points = 0;

    loop {
        clear_background(BLACK);

        if !has_lost {
            enemies.spawn();

            player.update();

            enemies.update(points);

            projectiles.update(&player);

            player.draw();

            enemies.draw();

            projectiles.fly();
            projectiles.draw();
            projectiles.clean();

            if enemies.check_projectile_collision(&mut projectiles) {
                points += 1;
            }

            if enemies.has_lost() {
                has_lost = true;
            }

            draw_text(format!("Points: {points}"), 0.0, 30.0, 40.0, DARKGRAY);

        }
        else {
            enemies.clear();
            points = 0;
            draw_text("YOU LOST YOU FOOL", 200.0, 300.0, 100.0, DARKGRAY);
            if is_key_down(KeyCode::Enter) {
                has_lost = false;
            }
        }



        next_frame().await;
    }

}
