pub struct Particle {
    pub px: i32,
    pub py: i32,
    pub vx: i32,
    pub vy: i32,
}

impl Particle {
    pub fn particle_move(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
    }

    pub fn v_update(&mut self, vx: i32, vy: i32) {
        self.vx += vx;
        self.vy += vy;
    }
}

fn occupied(x: i32, y: i32, queue: Vec<Particle>) -> bool {
    for p in queue.iter() {
        if x == p.px && y == p.py {
            return true;
        }
    }
    false
}
