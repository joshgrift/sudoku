extern crate termion;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use termion::color;

mod solver;

struct Stats {
  total: u32,
  solved: u32,
  failed: u32,
  failures: Vec<u32>
}

fn main() {
  print!("{}", color::Fg(color::LightBlue));
  println!("               _       _          \n              | |     | |         \n ___ _   _  __| | ___ | | ___   _ \n/ __| | | |/ _` |/ _ \\| |/ / | | |\n\\__ \\ |_| | (_| | (_) |   <| |_| |\n|___/\\__,_|\\__,_|\\___/|_|\\_\\___,_|");
  println!("                            solver\n");

  let args: Vec<String> = env::args().collect();
  let debug:bool = if args.len() > 2 { true } else {false};
  let mut stats: Stats = Stats { total: 0, solved: 0, failed: 0, failures: [].to_vec()};
  
  if debug {
    println!("{}Loading puzzles from {}", color::Fg(color::LightBlack), args[1]);
  } 

  let path = Path::new(&args[1]);
  let display = path.display();
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why),
    Ok(_) => (),
  }

  let mut puzzles = s.lines();
  let mut p = puzzles.next();

  while p != None {
    stats.total += 1;
    let mut puzzle = solver::Puzzle::load(p.unwrap());

    match puzzle.solve() {
      Err(_) => {
        if debug {
          println!("Failed to solve #{}", stats.total);
          puzzle.display();
        }
        stats.failures.push(stats.total);
        stats.failed +=1;
      },
      Ok(_) => stats.solved += 1,
    }

    p = puzzles.next();
  }
  
  println!("{}-----------------", color::Fg(color::White));
  println!("{} Attempted  {}", color::Fg(color::Blue), stats.total);
  println!("{} Failed     {}", color::Fg(color::LightRed), stats.failed);
  println!("{} Solved     {}", color::Fg(color::Green), stats.solved);
  println!("{}-----------------", color::Fg(color::White));
  println!("{} Success    {}%", color::Fg(color::White), (stats.solved as f32 / stats.total as f32 * 100 as f32).floor());
}