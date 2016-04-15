extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
  let cell_count = 100;
  let mut w = World::new(30, 30);

  for _ in 0..cell_count {

    // why do I need to do this?
    let width = w.width;
    let height = w.height;

    w.set(
      rand::thread_rng().gen_range(0, width),
      rand::thread_rng().gen_range(0, height),
      true
    );
  }

  loop {
    w = w.step();

    for y in 0..w.height {
      for x in 0..w.width {
        if w.get(x, y) {
          print!("🔶 ");
        } else {
          print!("  ");
        }
      }
      print!("\n")
    }

    thread::sleep(Duration::from_millis(250));

    // Hacky clear screen
    for _ in 0..80 {
      println!("");
    }
  }
}

struct World {
  cells: Vec<bool>,
  width: i32,
  height: i32,
}

impl World {
  fn new(width: i32, height: i32) -> World {
    let mut w = World {
      cells: Vec::new(),
      width: width,
      height: height,
    };

    for _ in 0..height {
      for _ in 0..width {
        w.cells.push(false);
      }
    }

    w
  }

  fn step(&mut self) -> World {
    let mut w2 = World::new(self.width, self.height);

    for y in 0..self.height {
      for x in 0..self.width {
        let count_neighbors = self.count_neighbors(x, y);
        let new_value = if self.get(x, y) {
          count_neighbors == 2 || count_neighbors == 3
        } else {
          count_neighbors == 3
        };
        w2.set(x, y, new_value)
      }
    }

    w2
  }

  fn set(&mut self, x: i32, y: i32, val: bool) {
    let idx = self.idx_from_coord(x, y);
    self.cells[idx as usize] = val
  }

  fn get(&mut self, x: i32, y: i32) -> bool {
    let idx = self.idx_from_coord(x, y);
    self.cells[idx as usize]
  }

  fn count_neighbors(&mut self, x: i32, y: i32) -> i32 {
    let neighbor_values = [
      self.get(x-1, y-1),
      self.get(x, y-1),
      self.get(x+1, y-1),

      self.get(x-1, y),
      self.get(x+1, y),

      self.get(x-1, y+1),
      self.get(x, y+1),
      self.get(x+1, y+1),
    ];

    let mut live_neighbor_count = 0;
    for v in &neighbor_values {
      if *v {
        live_neighbor_count += 1;
      }
    }

    return live_neighbor_count
  }

  // idx_from_coord takes x and y coordinates and returns the index in self.cells.
  // Coordinates wrap the boundaries of the world. e.g. Given World with a
  // width of 10, then an x coordinate of -1 should be equal to 9.
  fn idx_from_coord(&self, x: i32, y: i32) -> i32 {
    let mut x = x % self.width;
    if x < 0 {
      x += self.width;
    }
    let mut y = y % self.height;
    if y < 0 {
      y += self.height;
    }

    return y*self.width + x
  }
}

