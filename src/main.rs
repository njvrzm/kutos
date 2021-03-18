#![feature(assoc_char_funcs)]
#![feature(allocator_api)]
mod ca;
mod neighbor;

extern crate clap;
use clap::{Arg, App};
extern crate ctrlc;
use crate::ca::{World, Rule};
use crate::neighbor::Neighborhood;
use std::time::Duration;
use std::thread::sleep;
use std::process::exit;

fn main() {
    ctrlc::set_handler(move || {
        print!("{esc}[2J{esc}[?25h{esc}[1;1H", esc = 27 as char);
        exit(0);
    }).expect("Failed to set ctrl-c handler");
    let matches = App::new("kutos")
        .version("0.0.1")
        .author("Nathan Verzemnieks <njvrzm@gmail.com>")
        .about("A terminal cellular automaton runner")
        .arg(Arg::with_name("survival")
            .short("s")
            .long("survival")
            .value_name("SURVIVAL_COUNTS")
            .help("Specifies the neighbor counts letting a live cell survive"))
        .arg(Arg::with_name("birth")
            .short("b")
            .long("birth")
            .value_name("BIRTH_COUNTS")
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
    let rule = Rule::new(survival, birth);
    let mut world = World::new(111, 72, Neighborhood::Moore(2), rule);
    world.randomize(0.31);
    print!("{esc}[2J;{esc}[?25l", esc = 27 as char);

    for _ in 0..10000 {
        world.tick();
        sleep(Duration::new(0,10000000));
        world.display();
    }
}
