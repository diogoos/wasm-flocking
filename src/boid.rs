use rand::Rng;

use crate::quadtree::QuadTree;
use crate::vecmath::Vec2D;
use crate::geometry::Point;

const PERCEPTION_RADIUS: f64 = 25.;

pub struct Boid {
  pub position: Point,
  pub velocity: Vec2D,
  pub acceleration: Vec2D,
  max_force: f64,
  max_speed: f64,
}

pub struct BoidUpdate {
  pub alignment: Vec2D,
  pub separation: Vec2D,
  pub cohesion: Vec2D,
}

impl Boid {
  pub fn new(position: Point) -> Self {
    Self {
      position,
      velocity: Vec2D::random(rand::thread_rng().gen_range(2..4).into()),
      acceleration: Vec2D { x: 0., y: 0. },
      max_force: 0.2,
      max_speed: 5.,
    }
  }

  pub fn update(&self, qtree: &QuadTree) -> BoidUpdate {
    let mut update = BoidUpdate {
      alignment: Vec2D { x: 0., y: 0. },
      separation: Vec2D { x: 0., y: 0. },
      cohesion: Vec2D { x: 0., y: 0. },
    };
    let mut total = 0;

    let shift = ((PERCEPTION_RADIUS * PERCEPTION_RADIUS) / 2.).sqrt();

    // for other in flock {
    for other in qtree.query(
      &crate::geometry::Rect::new(
        crate::geometry::Point {
          x: self.position.x - shift,
          y: self.position.y - shift,
        },
        crate::geometry::Point {
          x: self.position.x + shift,
          y: self.position.y + shift,
        }
      )) {
      if other.position.x == self.position.x && other.position.y == self.position.y {
        continue; // skip self
      }

      // calculate alignment
      update.alignment.add(&other.velocity);

      // calculate separation
      let sqr_dst = self.position.sqr_dist(&other.position);
      let mut diff = Vec2D {
        x: self.position.x - other.position.x,
        y: self.position.y - other.position.y,
      };
      diff.normalize();
      diff.scalar_div(sqr_dst);
      // diff.scalar_div(sqr_dst);
      update.separation.add(&diff);

      // calculate cohesion
      let position = (&other.position).into();
      update.cohesion.add(&position);

      total += 1;
    }

    if total > 0 {
      update.alignment.scalar_div(f64::from(total));
      update.alignment.set_mag(self.max_speed);
      update.alignment.sub(&self.velocity);
      update.alignment.limit_mag(self.max_force);

      update.separation.scalar_div(f64::from(total));
      update.separation.set_mag(self.max_speed);
      update.separation.sub(&self.velocity);
      // update.separation.limit_mag(self.max_force);

      update.cohesion.scalar_div(f64::from(total));
      // update.cohesion.set_mag(self.max_speed);
      update.cohesion.sub(&self.velocity);
      update.cohesion.limit_mag(self.max_force);
    }

    update
  }

  pub fn check_edges(&mut self, max_width: f64, max_height: f64) {
    if self.position.x > max_width {
      self.position.x = 0.;
    } else if self.position.x < 0. {
      self.position.x = max_width;
    }

    if self.position.y > max_height {
      self.position.y = 0.;
    } else if self.position.y < 0. {
      self.position.y = max_height;
    }
  }

  pub fn step(&mut self) {
    // update position manually
    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;
    
    // update velocity
    self.velocity.add(&self.acceleration);
    self.velocity.limit_mag(self.max_speed);

    // reset acceleration
    self.acceleration.x = 0.;
    self.acceleration.y = 0.;
  }
}
