#![feature(assoc_char_funcs)]
#![feature(allocator_api)]
mod ca;

extern crate clap;
use clap::{Arg, App};
use crate::ca::{World, Rule};
use std::time::SystemTime;

fn main() {
    let matches = App::new("kutos")
        .version("0.0.1")
        .author("Nathan Verzemnieks <njvrzm@gmail.com>")
        .about("A terminal cellular automaton runner")
        .arg(Arg::with_name("survival")
            .short("s")
            .long("survival")
            .value_name("SURVIVAL")
            .help("Specifies the neighbor counts letting a live cell survive"))
        .arg(Arg::with_name("birth")
            .short("b")
            .long("birth")
            .value_name("BIRTH")
            .help("Specifies the neighbor counts letting an empty cell be born"))
        .arg(Arg::with_name("seed")
            .short("s")
            .long("seed")
            .value_name("SEED")
            .help("Specify a seed for the rng (default 37)"))
        .get_matches();
    let survival: Vec<i8> = matches
        .value_of("survival")
        .unwrap_or("2,3")
        .split(",")
        .map(|count|count.parse::<i8>().unwrap())
        .collect();
    let birth: Vec<i8> = matches
        .value_of("birth")
        .unwrap_or("3")
        .split(",")
        .map(|count|count.parse::<i8>().unwrap())
        .collect();
    let mut total = 0;
    let neighborhood = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let rule = Rule::new(survival, birth);
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
