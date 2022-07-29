#[derive(Clone, Debug)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

#[derive(Debug)]
pub struct Rect {
  pub(crate) bottom_left: Point,
  pub(crate) top_right: Point
}

impl Rect {
  pub fn new(bottom_left: Point, top_right: Point) -> Self {
    Self {
      bottom_left, top_right
    }
  }

  pub fn new_with_center(center: Point, width: u32, height: u32) -> Self {
    Self {
      bottom_left: Point { x: center.x - (width/2), y: center.y - (height/2) },
      top_right: Point { x: center.x + (width/2), y: center.y + (height/2) },
    }
  }

  // (10, 15)
  // bottom_left = (0, 800)
  // top_right = (800, 0)
  pub fn contains(&self, p: &Point) -> bool {
    p.x > self.bottom_left.x &&
      p.x < self.top_right.x &&
      p.y > self.bottom_left.y &&
      p.y < self.top_right.y
  }

  // NE, NW, SW, SE
  pub fn quadrants(&self) -> (Rect, Rect, Rect, Rect) {
    let center_x = (self.bottom_left.x + self.top_right.x) / 2;
    let center_y = (self.bottom_left.y + self.top_right.y) / 2;

    (
      Rect::new(Point {x: center_x, y: center_y},self.top_right.clone()),
      Rect::new(Point {x: self.bottom_left.x, y: center_y}, Point{x: center_x, y: self.top_right.y}),
      Rect::new(self.bottom_left.clone(), Point { x: center_x, y: center_y }),
      Rect::new(Point{x: center_x, y: self.bottom_left.y}, Point{x: self.top_right.x, y: center_y})
    )
  }


  pub fn intersects(&self, other: &Rect) -> bool {
    self.bottom_left.x < other.top_right.x &&
      self.top_right.x > other.bottom_left.x &&
      self.top_right.y > other.bottom_left.y &&
      self.bottom_left.y < other.top_right.y
  }

  pub fn is_inside(&self, of: &Rect) -> bool {
    of.contains(&self.bottom_left) && of.contains(&self.top_right)
  }
}