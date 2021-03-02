use rand::prelude::*;
use rand::distributions::Standard;
pub struct World {
    width: usize,
    height: usize,
    size: usize,
    cells: Vec<bool>,
    count: Vec<i8>,
    heat: Vec<i8>,
    neighborhood: Vec<(i32, i32)>,
}
impl World {
    pub fn new(width: usize, height: usize, neighborhood: Vec<(i32, i32)>) -> Self {
        Self {
            width,
            height,
            size: width * height,
            cells: vec![false; width * height],
            count: vec![0; width * height],
            heat: vec![0; width * height],
            neighborhood,
        }
    }
    pub fn randomize(&mut self, fill: f32) {
        let mut rng = StdRng::seed_from_u64(32);
        self.cells = vec![false; self.width * self.height];
        for index in 0..self.size {
            self.cells[index] = fill > rng.sample(Standard);

        }
        self.reset_count();
    }
    fn reset_count(&mut self) {
        self.count = vec![0; self.width * self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                self.update_count(x, y, false, self.cells[x + y * self.width])
            }
        }
    }
    fn update_count(&mut self, x: usize, y:usize, old_state: bool, new_state: bool) {
        if old_state != new_state {
            for (nx, ny) in self.neighbors(x, y) {
                self.count[nx + ny * self.width] += if new_state { 1 } else { -1 }
            }
        }
    }
    fn neighbors(&self, x: usize, y:usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        for (dx, dy) in &self.neighborhood {
            let nx = (((x + self.width) as i32 + dx) as usize)  % self.width;
            let ny = (((y + self.height) as i32 + dy) as usize) % self.height;
            neighbors.push((nx, ny))
        }
        neighbors
    }
    pub fn display(&self) {
        for y in 0..self.height {
             println!("{}", &self.cells[y*self.width..y*self.width+self.width].iter().map(|b|{if *b {'X'} else {' '}}).collect::<String>())
        }
        for y in 0..self.height {
            println!("{}", &self.count[y*self.width..y*self.width+self.width].iter().map(|c| char::from_digit(*c as u32, 10).unwrap()).collect::<String>())
        }
    }
}