pub struct Point {
  pub y: f32,
  pub x: f32
}

impl Point {
  pub fn calc_dis(&self, other: &Point) -> f32 {
    // Euclidean distance
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  }

  pub fn calc_dis_squared(&self, other: &Point) -> f32 {
    (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
  }

  pub fn zero() -> Point {
    Point {x: 0_f32, y: 0_f32}
  }

  pub fn new(x: f32, y: f32) -> Point {
    Point {x, y}
  }
}

impl Copy for Point {}
impl Clone for Point {
  fn clone(&self) -> Self {
    Point{x:self.x, y:self.y}
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == self.y
  }
}

pub fn interpret_input(raw: &str) -> (usize, Vec<Point>) {
  let mut lines = raw.lines();

  let mut max_path_length = 0;
  if let Some(line) = lines.next() {
    if let Ok(value) = line.trim().parse::<usize>() {
      max_path_length = value;
    }
  }

  let mut plants_count = 0;
  if let Some(line) = lines.next() {
    if let Ok(value) = line.trim().parse::<usize>() {
      plants_count = value;
    }
  }

  let mut plants = Vec::new();

  for _ in 0..plants_count {
    if let Some(line) = lines.next() {
      let mut parts = line.split_whitespace();

      parts.next();

      let x = match parts.next().and_then(|p| p.parse::<i32>().ok()) {
        Some(v) => v,
        None => continue,
      };

      let y = match parts.next().and_then(|p| p.parse::<i32>().ok()) {
        Some(v) => v,
        None => continue,
      };

      plants.push(Point {
        x: x as f32,
        y: y as f32,
      });
    }
  }

  (max_path_length, plants)
}