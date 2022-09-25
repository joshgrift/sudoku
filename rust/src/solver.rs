pub struct Puzzle {
  map: Vec<u32>,
  options: Vec<Vec<u32>>
}

const SIZE:usize = 9;

impl Puzzle {
  pub fn load(p:&str) -> Puzzle {
    let mut puzzle = Puzzle::make_empty();

    let mut x = 0;
    let mut y = 0;
    for c in p.chars() {
      if c != '.' {
        puzzle.set(x, y, c.to_digit(10).unwrap())
      }

      
      if x + 1 >= SIZE {
        x = 0;
        y += 1;
      } else {
        x += 1;
      }
    }

    return puzzle;
  }

  pub fn solve(&self) -> Result<u32, u32> {
    return Result::Err(0);
  }

  fn make_empty() -> Puzzle {
    let mut options: Vec<Vec<u32>> = Vec::new();
    let mut map: Vec<u32> = vec![];

    for i in 0..SIZE {
      options.push(Vec::new());
      for j in 0..SIZE {
        map.push(0);
        options.push(Vec::new());
        for k in 0..SIZE {
          options[i * SIZE + j].push((k + 1) as u32);
        }
      } 
    }

    return Puzzle {
      map,
      options
    };
  }

  pub fn display(&self) {
    print!("-------------------------------------\n");
    for i in 0..SIZE {
      for j in 0..SIZE {
        let p = self.map[i * SIZE + j];
        if p != 0 {
          print!("| {} ", p);
        } else {
          print!("|   ");
        }
      } 
      print!("| \n-------------------------------------\n");
    }
  }

  fn get(&self, x: usize, y: usize) -> u32{
    return self.map[x + y * SIZE];
  }

  fn get_options(&self, x: usize, y: usize) -> &Vec<u32>{
    return &self.options[x + y * SIZE];
  }

  fn set(&mut self, x: usize, y: usize, v: u32) {
    self.map[x + y * SIZE] = v;

    // TODO: remove options
  }
}