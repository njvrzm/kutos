use rand::prelude::*;
use rand::distributions::Standard;
use crate::neighbor::Neighborhood;

type Place = (usize, usize);

pub struct Rule {
    survival: Vec<bool>,
    birth: Vec<bool>,
}

impl Rule {
    pub fn new(survive_counts: Vec<i8>, birth_counts: Vec<i8>) -> Self {
        let mut survive = vec![false; 9];
        for c in survive_counts {
            survive[c as usize] = true;
        }
        let mut born = vec![false; 9];
        for c in birth_counts {
            born[c as usize] = true;
        }
        Self{survival: survive, birth: born}
    }
    fn survives(&self, count: i8) -> bool {
        self.survival[count as usize]
    }
    fn is_born(&self, count: i8) -> bool {
        self.birth[count as usize]
    }
}

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    count: Vec<i8>,
    born: Vec<(usize, usize)>,
    died: Vec<(usize, usize)>,
    last_change: Vec<i32>,
    neighborhood: Neighborhood,
    rule: Rule,
    generation: i32,
}

impl World {
    pub fn new(width: usize, height: usize, neighborhood: Neighborhood, rule: Rule) -> Self {
        Self{
            width,
            height,
            cells: vec![false; width * height],
            count: vec![0; width * height],
            born: Vec::with_capacity(width * height),
            died: Vec::with_capacity(width * height),
            last_change: vec![0; width * height],
            neighborhood,
            rule,
            generation: 0,
        }
    }
    pub fn reset(&mut self) {
        self.cells = vec![false; self.width * self.height];
        self.count = vec![0; self.width * self.height];
        self.last_change = vec![0; self.width * self.height];
        self.generation = 0;
    }
    pub fn randomize(&mut self, fill: f32, seed: Option<u64>) {
        let mut rng = StdRng::seed_from_u64(seed.unwrap_or(37));
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
        // let current = self.cells[index];
        if self.cells[index] != alive {
            self.cells[index] = alive;
            self.last_change[index] = self.generation;
            self.update_neighbors(x, y, alive);
        }
    }
    fn update_neighbors(&mut self, x: usize, y: usize, alive: bool) {
        let w = self.width as i32;
        let h = self.height as i32;
        let s = w * h;
        let i = (x + y * self.width) as i32;
        let delta = if alive {1} else {-1};
        for dx in -1..=1 as i32 {

            for dy in -1..=1 as i32{
                let mut ni: i32 = i + dx + dy * w;
                if dx == 0 && dy == 0 {
                    continue
                }
                if dx < 0  && x < -dx as usize {
                    ni += w;
                } else if dx > 0 && x >= self.width - (dx as usize) {
                    ni -= w;
                }
                if dy < 0 && y < -dy as usize {
                    ni += s;
                } else if dy > 0 && y >= self.height - (dy as usize) {
                    ni -= s;
                }
                self.count[ni as usize] += delta;
            }
        }

    }
    pub fn tick(&mut self) {
        self.generation += 1;
        self.born.clear();
        self.died.clear();
        // let mut born: Vec<Place> = Vec::with_capacity(self.width * self.height);
        // let mut died: Vec<Place> = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            let offset = y * self.width;
            for x in 0..self.width {
                if self.cells[offset + x] {
                    if !self.rule.survives(self.count[offset + x]) {
                        self.died.push((x, y))
                    }
                } else {
                    if self.rule.is_born(self.count[offset + x]) {
                        self.born.push((x, y))
                    }
                }
            }
        }
        // print!("Born: {}{esc}[K\nDied: {}{esc}[K", born.len(), died.len(), esc = 27 as char);
        for (x, y) in self.born.clone() {
            self.set_cell_state(x, y, true);
        }
        for (x, y) in self.died.clone() {
            self.set_cell_state(x, y, false);
        }
    }
    pub fn evaluate(&self) -> Kind {
        Kind::Unknown
    }
    pub fn display(&self) {
        print!("{esc}[1;1H", esc = 27 as char);
        for y in (0..self.height).step_by(2) {
            let mut row: Vec<&str> = Vec::with_capacity(self.width);
            for x in 0..self.width {
                row.push(match (self.cells[y*self.width + x], self.cells[(y+1)*self.width + x]) {
                    (true, true) => "█",
                    (false, true) => "▄",
                    (true, false) => "▀",
                    (false, false) => " ",
                });
            }
            println!("{}", row.join(""));
        }
    }
}

pub enum Kind {
    Unknown,
    Interesting,
    Dead,
    Exploding,
}

