

struct Movement {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32
}

impl Movement {

    fn getX(&self) -> f32 {
        self.x
    }
    
    fn getY(&self) -> f32 {
        self.y
    }

    fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

}