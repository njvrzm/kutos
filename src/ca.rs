use rand::prelude::*;
use rand::distributions::Standard;

type Place = (usize, usize);

pub struct Rule {
    survive_counts: Vec<i8>,
    birth_counts: Vec<i8>,
}

impl Rule {
    pub fn new(survive_counts: Vec<i8>, birth_counts: Vec<i8>) -> Self {
        Self{survive_counts, birth_counts}
    }
    fn survives(&self, count: i8) -> bool {
        self.survive_counts.contains(&count)
    }
    fn is_born(&self, count: i8) -> bool {
        self.birth_counts.contains(&count)
    }
}

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    count: Vec<i8>,
    last_change: Vec<i32>,
    neighborhood: Vec<(i32, i32)>,
    rule: Rule,
    generation: i32,
}

impl World {
    pub fn new(width: usize, height: usize, neighborhood: Vec<(i32, i32)>, rule: Rule) -> Self {
        Self{
            width,
            height,
            cells: vec![false; width * height],
            count: vec![0; width * height],
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
    pub fn randomize(&mut self, fill: f32) {
        let mut rng = StdRng::seed_from_u64(37);
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
            self.update_neighbor_counts(x, y, alive);
        }
    }
    fn update_neighbor_counts(&mut self, x: usize, y: usize, alive: bool) {
        let w = self.width as i32;
        let h = self.height as i32;
        let s = w * h;
        let i = (x + y * self.width) as i32;
        let delta = if alive {1} else {-1};
        for (dx, dy) in self.neighborhood.iter() {
            let mut ni: i32 = i + dx + dy * w;
            if *dx < 0  && x == 0 {
                ni += w;
            } else if *dx > 0 && x == self.width - 1 {
                ni -= w;
            }
            if *dy < 0 && y == 0 {
                ni += s;
            } else if *dy > 0 && y == self.height - 1 {
                ni -= s;
            }
            self.count[ni as usize] += delta;
        }

    }
    fn wrap(&self, x: &usize, y: &usize, dx: &i32, dy: &i32) -> (usize, usize) {
        ((((x + self.width) as i32 + dx) as usize) % self.width, (((y + self.height) as i32 + dy) as usize) % self.height)
    }
    pub fn tick(&mut self) {
        self.generation += 1;
        let mut born: Vec<Place> = Vec::with_capacity(self.width * self.height);
        let mut died: Vec<Place> = Vec::with_capacity(self.width * self.height);
        for y in 0..self.height {
            let offset = y * self.width;
            for x in 0..self.width {
                if self.cells[offset + x] {
                    if !self.rule.survives(self.count[offset + x]) {
                        died.push((x, y))
                    }
                } else {
                    if self.rule.is_born(self.count[offset + x]) {
                        born.push((x, y))
                    }
                }
            }
        }
        print!("Born: {}{esc}[K\nDied: {}{esc}[K", born.len(), died.len(), esc = 27 as char);
        for (x, y) in born {
            self.set_cell_state(x, y, true);
        }
        for (x, y) in died {
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
        // for y in 0..self.height {
        //     println!("{}", &self.count[y*self.width..y*self.width+self.width].iter().map(|c| char::from_digit(*c as u32, 10).unwrap()).collect::<String>())
        // }
    }
}

pub enum Kind {
    Unknown,
    Interesting,
    Dead,
    Exploding,
}

