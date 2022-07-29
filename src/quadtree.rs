use std::vec;
use super::geometry::*;

#[derive(Clone)]
pub struct QuadItem<'a> {
  pub p: Point,
  pub boid_ptr: &'a crate::boid::Boid,
}

pub enum QuadTreeResult {
  Ok, Err
}


pub struct QuadTree<'a> {
  pub(crate) boundry: Rect,
  capacity: u32,

  items: Vec<QuadItem<'a>>,

  // store recursive quadtrees on heap
  pub(crate) northeast: Option<Box<QuadTree<'a>>>,
  pub(crate) northwest: Option<Box<QuadTree<'a>>>,
  pub(crate) southwest: Option<Box<QuadTree<'a>>>,
  pub(crate) southeast: Option<Box<QuadTree<'a>>>,
}

impl<'a> QuadTree<'a> {
  pub fn new(boundry: Rect, capacity: u32) -> Self {
    QuadTree { boundry, capacity, items: vec![], northeast: None, northwest: None, southwest: None, southeast: None }
  }

  pub fn insert(&mut self, point: QuadItem<'a>) -> QuadTreeResult {
    // don't insert point if not within boundry (important for recursion)
    if !self.boundry.contains(&point.p) { return QuadTreeResult::Err }

    // if within capacity, just insert here
    if self.items.len() < (self.capacity as usize) {
      self.items.push(point);
      return QuadTreeResult::Ok;
    }

    // subdivide quadtree (if not already divded)
    if let None = self.northeast {
      self.subdivide();
    }

    // move point to children
    if self.northeast.as_ref().unwrap().boundry.contains(&point.p) {
      return self.northeast.as_mut().unwrap().insert(point);
    } else if self.northwest.as_ref().unwrap().boundry.contains(&point.p) {
      return self.northwest.as_mut().unwrap().insert(point);
    } else if self.southwest.as_ref().unwrap().boundry.contains(&point.p) {
      return self.southwest.as_mut().unwrap().insert(point);
    } else {
      return self.southeast.as_mut().unwrap().insert(point);
    }
  }


  pub fn query(&self, range: &Rect) -> Vec<QuadItem> {
    let mut found = vec![];
    self._query(range, &mut found);
    found
  }

  pub fn clear(&mut self) {
    self.items = vec![];
    self.northeast = None;
    self.northwest = None;
    self.southeast = None;
    self.southwest = None;
  }

  fn _query(&self, range: &Rect, found: &mut Vec<QuadItem<'a>>) {
    if !range.intersects(&self.boundry) { return }

    if self.boundry.is_inside(range) {
      // if quad is completely inside range, add add everything
      found.append(&mut self.items.clone());
    } else {
      // otherwise, check point-by-point
      for p in &self.items {
        if range.contains(&p.p) {
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

  pub fn all(&self) -> Vec<QuadItem<'a>> {
    let mut found = vec![];
    self._all(&mut found);
    found
  }

  fn _all(&self, found: &mut Vec<QuadItem<'a>>) {
    found.append(&mut self.items.clone());

    if let Some(_) = self.northeast {
      self.northeast.as_ref().unwrap()._all(found);
      self.northwest.as_ref().unwrap()._all(found);
      self.southwest.as_ref().unwrap()._all(found);
      self.southeast.as_ref().unwrap()._all(found);
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