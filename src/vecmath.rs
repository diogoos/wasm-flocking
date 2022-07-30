use std::f64::consts::PI;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Vec2D {
  pub x: f64,
  pub y: f64,
}

impl Vec2D {
  // SCALAR OPERATIONS
  pub fn scalar_mult(&mut self, s: f64) {
    self.x *= s;
    self.y *= s;
  }
  pub fn scalar_div(&mut self, s: f64) {
    self.x *= 1./s;
    self.y *= 1./s;
  }

  // VECTOR OPERATIONS
  pub fn add(&mut self, v: &Vec2D) {
    self.x += v.x;
    self.y += v.y;
  }
  pub fn sub(&mut self, v: &Vec2D) {
    self.x -= v.x;
    self.y -= v.y;
  }

  
  // MAGNITUDE
  fn mag_sq(&self) -> f64 {
    (self.x * self.x) + (self.y * self.y)
  }
  fn mag(&self) -> f64 {
    self.mag_sq().sqrt()
  }
  pub fn set_mag(&mut self, mag: f64) {
    let len = self.mag();
    if len != 0. {
      self.scalar_mult(mag / len)
    }
  }
  pub fn limit_mag(&mut self, max: f64) {
    let msq = self.mag_sq();
    if msq > (max * max) {
      self.scalar_div(msq.sqrt()); // normalize it
      self.scalar_mult(max); // make max value
    }
  }

  pub fn random(len: f64) -> Self {
    let r: f64 = rand::thread_rng().gen_range(0.0..1.0) * 2.0 * PI;
    Self {
      x: len * r.cos(),
      y: len * r.sin()
    }
  }

  pub fn normalize(&mut self) {
    self.scalar_div(self.mag())
  }
}

// conversion from point
impl From<&crate::geometry::Point> for Vec2D {
    fn from(p: &crate::geometry::Point) -> Self {
      Vec2D { x: p.x, y: p.y }
    }
}