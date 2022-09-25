use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

mod solver;

struct Stats {
  total: u32,
  solved: u32,
  failed: u32,
  failures: Vec<u32>
}

fn main() {
  const DEBUG:bool = true;
  let mut stats: Stats = Stats { total: 0, solved: 0, failed: 0, failures: [].to_vec()};

  let args: Vec<String> = env::args().collect();
  println!("Testing {}.", args[1]);

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
        if DEBUG {
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

  if DEBUG {
    //println!("Failed to solve {:?}", stats.failures);
  }

  println!("Attempted  {}", stats.total);
  println!("Solved     {}", stats.solved);
  println!("Failed     {}", stats.failed);
}