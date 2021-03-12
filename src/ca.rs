use rand::prelude::*;
use rand::distributions::Standard;
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    delta: Vec<i8>,
    count: Vec<i8>,
    last_change: Vec<i32>,
    neighborhood: Vec<(i32, i32)>,
    generation: i32,
}

pub enum State {
    Unknown,
    Interesting,
    Dead,
    Exploding,
}

impl World {
    pub fn new(width: usize, height: usize, neighborhood: Vec<(i32, i32)>) -> Self {
        Self{
            width,
            height,
            cells: vec![false; width * height],
            delta: vec![0; width * height],
            count: vec![0; width * height],
            last_change: vec![0; width * height],
            neighborhood,
            generation: 0,
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height, self.neighborhood.clone());
    }
    pub fn randomize(&mut self, fill: f32) {
        let mut rng = StdRng::seed_from_u64(32);
        self.reset();
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
        // let current = self.cells[index];
        if self.cells[index] != alive {
            self.cells[index] = alive;
            self.last_change[index] = self.generation;
            for (dx, dy) in self.neighborhood.iter() {
                let (nx, ny) = self.wrap(&x, &y, dx, dy);
                self.count[nx + ny * self.width] += delta;
            }
        }
    }
    fn wrap(&self, x: &usize, y: &usize, dx: &i32, dy: &i32) -> (usize, usize) {
        ((((x + self.width) as i32 + dx) as usize) % self.width, (((y + self.height) as i32 + dy) as usize) % self.height)
    }
    pub fn tick(&mut self) {
        self.generation += 1;
        // let mut born = 0;
        // let mut died = 0;
        for index in 0..self.width * self.height {
            self.delta[index] = match (self.cells[index], self.count[index]) {
                (true, 2) | (true, 3) => 0,
                (true, _) => -1,
                (false, 3) => 1,
                (false, _) => 0,
            }
        }
        for y in 0..self.height {
            let offset = y * self.width;
            for x in 0..self.width {
                let delta = self.delta[offset + x];
                if delta != 0 {
                    self.set_cell_state(x, y, if delta > 0 {true} else {false});
                }
            }
        }
    }
    pub fn evaluate(&self) -> State {
        State::Unknown
    }
    pub fn display(&self) {
        for y in 0..self.height {
             println!("{}", &self.cells[y*self.width..y*self.width+self.width].iter().map(|b|{if *b {'X'} else {' '}}).collect::<String>())
        }
        // for y in 0..self.height {
        //     println!("{}", &self.count[y*self.width..y*self.width+self.width].iter().map(|c| char::from_digit(*c as u32, 10).unwrap()).collect::<String>())
        // }
    }
}