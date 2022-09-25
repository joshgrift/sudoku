pub struct Puzzle {
  map: Vec<u32>,
  options: Vec<Vec<u32>>,
  placed: usize
}

const SIZE:usize = 9;
const SQUARE:usize = 3;

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

  pub fn solve(&mut self) -> Result<u32, u32> {
    let mut action_taken = false;

    while !self.is_finished() {
      for x in 0..SIZE {
        for y in 0..SIZE {
          if self.get_options(x, y).len() == 1 {
            self.set(x, y, self.get_options(x, y)[0]);
          }
        }
      }

      // if we havn't done anything, escape the loop
      if !action_taken {
        return Result::Err(0);
      } else {
        action_taken = true;
      }
    }

    return Result::Ok(1);
  }

  pub fn is_finished(&self) -> bool {
    if self.placed >= SIZE * SIZE {
      return true; 
    }
    return false;
  }

  fn make_empty() -> Puzzle {
    let mut empty_options: Vec<u32> = vec![];
    for k in 0..SIZE {
      empty_options.push((k + 1) as u32);
    }

    return Puzzle {
      map: vec![0; SIZE * SIZE],
      options: vec![empty_options.clone(); SIZE * SIZE],
      placed: 0
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
    println!("{:?}", self.options);
  }

  fn get(&self, x: usize, y: usize) -> u32{
    return self.map[x + y * SIZE];
  }

  fn set(&mut self, x: usize, y: usize, v: u32) {
    self.map[x + y * SIZE] = v;

    self.clear_options(x, y);

    // clear x
    for i in 0..SIZE {
      self.remove_option(i, y, v);
    }

    // clear y
    for j in 0..SIZE {
      self.remove_option(x, j, v);
    }

    // clear the square we are in now
    let sx = x / SQUARE;
    let sy = y / SQUARE;

    for i in 0..SQUARE {
      for j in 0..SQUARE {
        self.remove_option(sx * SQUARE + j, sy * SQUARE + i, v);
      }
    }
  }

  fn get_options(&self, x: usize, y: usize) -> &Vec<u32>{
    return &self.options[x + y * SIZE];
  }

  fn remove_option(&mut self, x: usize, y: usize, v: u32) -> &Vec<u32>{
    let i = x + y * SIZE;

    match self.options[i].iter().position(|&a| a == v) {
      Some(index) => { self.options[i].remove(index); }
      None => (),
    }
    
    return &self.options[i];
  }

  fn clear_options(&mut self, x: usize, y: usize) -> &Vec<u32>{
    let i = x + y * SIZE;
    self.options[i].clear();
    return &self.options[i];
  }
}