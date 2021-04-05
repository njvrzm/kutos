use rand::prelude::*;
use rand::distributions::Standard;
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    born: Vec<usize>,
    died: Vec<usize>,
    count: Vec<i32>,
    neighborhood: Vec<(i32, i32)>,
    generation: u64,
}

impl World {
    pub fn new(width: usize, height: usize, neighborhood: Vec<(i32, i32)>) -> Self {
        let size = width * height;
        Self{
            width,
            height,
            cells: vec![false; size],
            born: Vec::with_capacity(size),
            died: Vec::with_capacity(size),
            count: vec![0; size],
            neighborhood,
            generation: 0,
        }
    }
    pub fn randomize(&mut self, fill: f32) {
        let mut rng = StdRng::seed_from_u64(32);
        for x in 0..self.width {
            for y in 0..self.height {
                if fill > rng.sample(Standard) {
                    self.set_cell_state(x, y, true)
                }
            }
        }
    }
    fn set_cell_state(&mut self, x: usize, y: usize, alive: bool) {
        let index = x + y * self.width;
        let delta = if alive {1} else {-1};
        if self.cells[index] != alive {
            self.cells[index] = alive;
            for (dx, dy) in &self.neighborhood {
                let mut nx = *dx + x as i32;
                let mut ny = *dy + y as i32;
                if nx < 0 {
                    nx += self.width as i32
                } else if nx > self.width as i32 {
                    nx -= self.width as i32
                }
                if ny < 0 {
                    ny += self.height as i32
                } else if ny > self.height as i32 {
                    ny -= self.height as i32
                }

                self.count[nx as usize + (ny as usize) * self.width] += delta;
            }
        }
    }
    pub fn tick(&mut self) {
        self.generation += 1;
        self.born.clear();
        self.died.clear();
        for index in 0..self.width * self.height {
            match (self.cells[index], self.count[index]) {
                (true, 2) | (true, 3) => (),
                (true, _) => self.died.push(index),
                (false, 3) => self.born.push(index),
                (false, _) => (),
            }
        }
        // this works but defeats the purpose
        //for index in self.died.clone() {
        // This gives "cannot borrow `*self` as mutable because it is also borrowed as immutable"
        for index in self.died.iter() {
            self.set_cell_state(*index%self.width, *index/self.width, false);
        }
        for index in self.born.iter() {
            self.set_cell_state(*index%self.width, *index/self.width, true);

        }
    }
}