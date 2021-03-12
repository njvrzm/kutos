#![feature(assoc_char_funcs)]
#![feature(allocator_api)]
mod ca;

use crate::ca::World;
use std::time::SystemTime;

fn main() {
    let mut total = 0;
    let neighborhood = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut world = World::new(256, 256, neighborhood.clone());
    for _ in 0..10 {
        world.randomize(0.31);
        let now = SystemTime::now();
        for _ in 0..100 {
            world.tick();
            // world.display();
            // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        total += now.elapsed().unwrap().as_micros();
    }
    println!("{}", total/1000);
    // for (i, v) in world.heatmap().iter().enumerate() {
    //     println!("{}\t{}", (i as i32) - 128, v);
    // }
}
