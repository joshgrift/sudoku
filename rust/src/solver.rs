pub struct Puzzle {
  raw: Vec<u32>
}

impl Puzzle {
  pub fn load(p:&str) -> Puzzle {
    let mut puzzle = Puzzle {raw:[].to_vec()};

    for c in p.chars() {
      if c == '.' {
        puzzle.raw.push(0);
      } else {
        puzzle.raw.push(c.to_digit(10).unwrap())  
      }
      
    }

    return puzzle;
  }

  pub fn solve(&self) -> Result<u32, u32> {
    return Result::Err(0);
  }

  pub fn display(&self) {
    print!("-------------------------------------\n");
    for i in 0..9 {
      for j in 0..9 {
        let p = self.raw[i * 9 + j];
        if p != 0 {
          print!("| {} ", p);
        } else {
          print!("|   ");
        }
      } 
      print!("| \n-------------------------------------\n");
    }
  }
}