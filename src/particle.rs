use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Particle {
    pub px: i32,
    pub py: i32,
    pub vx: i32,
    pub vy: i32,
}

impl Particle {
    
    pub fn update(&mut self, occupied: &mut HashSet<(i32, i32)>) {
        let mut dirs = vec![-1, 0, 1];
        let mut done = false;
        dirs.shuffle(&mut rand::thread_rng());
        
        while dirs.len() > 0 && !done {
            let dir = match dirs.pop() {
                Some(i) => i,
                None => 0
            };
            if !occupied.contains(&(self.px + dir, self.py - 1)) {
                occupied.remove(&(self.px, self.py));
                self.px += dir;
                self.py -= 1;
                done = true;
                occupied.insert((self.px, self.py));
            }
        }
    }
    
    pub fn shift(&mut self, new_x: i32, new_y: i32, occupied: &mut HashSet<(i32, i32)>) {
        occupied.remove(&(self.px, self.py));
        self.px = new_x;
        self.py = new_y;
        occupied.insert((self.px, self.py));
    }
    
}

