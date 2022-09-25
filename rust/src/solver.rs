pub struct Puzzle {
  map: Vec<u32>,
  options: Vec<Vec<u32>>,
  placed: usize
}

struct Tile {
  num: u32,
  count: u32,
  x: usize,
  y: usize,
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
          // if there is only one option in a tile, set that tile to the option
          if self.get_options(x, y).len() == 1 {
            self.set(x, y, self.get_options(x, y)[0]);
            action_taken = true;
          }
        }
      }

      // check for single empty tiles horizontally 
      for y in 0..SIZE {
        let mut row: Vec<u32> = Puzzle::get_empty_row();
        let mut new_x:usize = 0;

        for x in 0..SIZE {
          match row.iter().position(|&a| a == self.get(x, y)) {
            Some(index) => { 
              row.remove(index); 
              
            },
            None => {
              new_x = x; 
            }
          }
        }

        if row.len() == 1 {
          self.set(new_x, y, row[0]);
          action_taken = true;
        }
      }

      // check for single empty tile vertically
      for x in 0..SIZE {
        let mut column: Vec<u32> = Puzzle::get_empty_row();
        let mut new_y:usize = 0;

        for y in 0..SIZE {
          match column.iter().position(|&a| a == self.get(x, y)) {
            Some(index) => { 
              column.remove(index); 
            }
            None => {
              new_y = y; 
            },
          }
        }

        if column.len() == 1 {
          self.set(x, new_y, column[0]);
          action_taken = true;
        }
      }

      // check for one option in a square
      for i in 0..SQUARE {
        for j in 0..SQUARE {
          let mut candidates:Vec<Tile> = vec![];

          for sx in 0..SQUARE {
            for sy in 0..SQUARE {
              let x = i * SQUARE + sx; 
              let y = j * SQUARE + sy;
          
              match candidates.iter().position(|a| a.num == self.get(x, y)) {
                Some(index) => { 
                  candidates[index].count += 1;
                }
                None => {
                  candidates.push(Tile {num: self.get(x, y), count: 1, x, y})
                },
              }
            }
          }

          for i in candidates {
            if i.count == 1 {
              self.set(i.x, i.y, i.num);
              action_taken = true;
            }
          }
        }
      }

      // if we haven't done anything, escape the loop
      if !action_taken {
        return Result::Err(0);
      } else {
        action_taken = false;
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

  fn get_empty_row() -> Vec<u32> {
    return vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
  }

  fn make_empty() -> Puzzle {
    return Puzzle {
      map: vec![0; SIZE * SIZE],
      options: vec![Puzzle::get_empty_row(); SIZE * SIZE],
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

    self.placed += 1;
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