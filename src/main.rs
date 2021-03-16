#![feature(assoc_char_funcs)]
#![feature(allocator_api)]
mod ca;

use crate::ca::{World, Rule};
use std::time::SystemTime;

fn main() {
    let mut total = 0;
    let neighborhood = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let rule = Rule::new(vec![1,2,3], vec![1,3]);
    let mut world = World::new(70, 70, neighborhood.clone(), rule);
    world.randomize(0.31);
    let now = SystemTime::now();
    for _ in 0..10000 {
        world.tick();
        world.display();
    }
    total += now.elapsed().unwrap().as_micros();
    println!("{}", total/100);
    // for (i, v) in world.heatmap().iter().enumerate() {
    //     println!("{}\t{}", (i as i32) - 128, v);
    // }
}
