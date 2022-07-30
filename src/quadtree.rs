use std::vec;
use super::geometry::*;
use super::boid::*;

pub enum QuadTreeResult {
  Ok, Err
}

pub struct QuadTree<'a> {
  pub(crate) boundry: Rect,
  capacity: u32,

  boids: Vec<&'a Boid>,

  // store recursive quadtrees on heap
  pub(crate) northeast: Option<Box<QuadTree<'a>>>,
  pub(crate) northwest: Option<Box<QuadTree<'a>>>,
  pub(crate) southwest: Option<Box<QuadTree<'a>>>,
  pub(crate) southeast: Option<Box<QuadTree<'a>>>,
}

impl<'a> QuadTree<'a> {
  pub fn new(boundry: Rect, capacity: u32) -> Self {
    QuadTree { boundry, capacity, boids: vec![], northeast: None, northwest: None, southwest: None, southeast: None }
  }

  pub fn insert(&mut self, boid: &'a Boid) -> QuadTreeResult {
    // don't insert point if not within boundry (important for recursion)
    if !self.boundry.contains(&boid.position) { return QuadTreeResult::Err }

    // if within capacity, just insert here
    if self.boids.len() < (self.capacity as usize) {
      self.boids.push(boid);
      return QuadTreeResult::Ok;
    }

    // subdivide quadtree (if not already divded)
    if let None = self.northeast {
      self.subdivide();
    }

    // move point to children
    if self.northeast.as_ref().unwrap().boundry.contains(&boid.position) {
      return self.northeast.as_mut().unwrap().insert(boid);
    } else if self.northwest.as_ref().unwrap().boundry.contains(&boid.position) {
      return self.northwest.as_mut().unwrap().insert(boid);
    } else if self.southwest.as_ref().unwrap().boundry.contains(&boid.position) {
      return self.southwest.as_mut().unwrap().insert(boid);
    } else {
      return self.southeast.as_mut().unwrap().insert(boid);
    }
  }


  pub fn query(&self, range: &Rect) -> Vec<&Boid> {
    let mut found = vec![];
    self._query(range, &mut found);
    found
  }

  pub fn clear(&mut self) {
    self.boids = vec![];
    self.northeast = None;
    self.northwest = None;
    self.southeast = None;
    self.southwest = None;
  }

  fn _query(&self, range: &Rect, found: &mut Vec<&'a Boid>) {
    if !range.intersects(&self.boundry) { return }

    if self.boundry.is_inside(range) {
      // if quad is completely inside range, add add everything
      found.append(&mut self.boids.clone());
    } else {
      // otherwise, check point-by-point
      for p in &self.boids {
        if range.contains(&p.position) {
          found.push(p.clone());
        }
      }
    }

    if let Some(_) = self.northeast {
      self.northeast.as_ref().unwrap()._query(range, found);
      self.northwest.as_ref().unwrap()._query(range, found);
      self.southwest.as_ref().unwrap()._query(range, found);
      self.southeast.as_ref().unwrap()._query(range, found);
    }
  }

  fn subdivide(&mut self) {
    if let Some(_) = self.northeast { return } // don't subdivide twice
    let boundries = self.boundry.quadrants();
    
    self.northeast = Some(Box::new(QuadTree::new(boundries.0, self.capacity)));
    self.northwest = Some(Box::new(QuadTree::new(boundries.1, self.capacity)));
    self.southwest = Some(Box::new(QuadTree::new(boundries.2, self.capacity)));
    self.southeast = Some(Box::new(QuadTree::new(boundries.3, self.capacity)));

    // move points to sub divisions
    // let old_points = mem::replace(&mut self.points, Vec::new());
    // for p in old_points {
    //   self.insert(p);
    // }
  }
}