#![feature(assoc_char_funcs)]
#![feature(allocator_api)]
mod ca;

use crate::ca::World;
use std::time::SystemTime;

fn main() {
    let neighborhood = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut world = World::new(256, 256, neighborhood.clone());
    world.randomize(0.31);
    let now = SystemTime::now();
    for _ in 0..1000 {
        world.tick();
    }
    println!("{}", now.elapsed().unwrap().as_micros());
}
