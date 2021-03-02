#![feature(assoc_char_funcs)]
mod ca;
use crate::ca::World;

fn main() {
    let neighborhood = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut world = World::new(100, 100, neighborhood);
    world.randomize(0.3);
    world.display();
}
