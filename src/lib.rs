extern crate console_error_panic_hook;

use std::vec;

use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

pub mod quadtree;
use quadtree::QuadTree;
pub mod geometry;
use geometry::{Rect, Point};

pub mod boid;
mod vecmath;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

pub static mut flock: Vec<boid::Boid> = vec![];

#[wasm_bindgen]
pub struct FlockingApp {
  canvas: web_sys::HtmlCanvasElement,
  context: CanvasRenderingContext2d,
  qtree: QuadTree<'static>,
}




#[wasm_bindgen]
impl FlockingApp {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    // setup error logging
    console_error_panic_hook::set_once();

    // load canvas context
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

    let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    let bounds = Rect::new(
      Point { x: 0., y: 0. },
      Point { x: canvas.width().into(), y: canvas.height().into() }
    );

    let qtree = QuadTree::new(
      bounds,
      6
    );

    let mut rand = rand::thread_rng();
    
    for _ in 0..300 {
      unsafe {
        flock.push(boid::Boid::new(Point {
          x: rand.gen_range(0..canvas.width()).into(),
          y: rand.gen_range(0..canvas.height()).into(),
        }))
      }
    }

    FlockingApp {
      qtree,
      canvas,
      context,
    }
  }

  pub fn draw(&mut self) {
    // clear frame
    self.context.set_fill_style(&JsValue::from_str("white"));
    self.context.fill_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);
    self.context.set_fill_style(&JsValue::from_str("black"));

    // reset qt
    self.qtree.clear();

    unsafe {    
      for boid in &flock {
        self.qtree.insert(quadtree::QuadItem {
          p: boid.position.clone(),
          boid_ptr: &boid,
        });
      }
    // for (const boid of flock) {
    //   quadTree.addItem(boid.position.x, boid.position.y, boid);
    // }

    // draw points
      for i in 0..flock.len() {
        flock[i].check_edges(self.canvas.width().into(), self.canvas.height().into());

        // calculate parameters
        let update = &mut flock[i].update(&self.qtree);
        // log(format!("Align: {:?}, Cohesion: {:?}, Separation: {:?}", update.alignment, update.cohesion, update.separation).as_str());
        update.alignment.scalar_mult(1.5);
        update.separation.scalar_mult(0.01);
        
        // update the boid's parameters
        let boid = &mut flock[i];
        boid.acceleration.add(&update.alignment);
        boid.acceleration.add(&update.cohesion);
        boid.acceleration.add(&update.separation);
        

        // prepare and render
        boid.step();
        
        self.context.fill_rect(boid.position.x, boid.position.y, 5., 5.);
      }
    }

    // draw the clusters
    self.qtree.draw(&self.context);
  }
}

impl<'a> QuadTree<'a> {
  fn draw(&self, context: &CanvasRenderingContext2d) {
    match self.northeast {
      Some(_) => {
        let quads = self.boundry.quadrants();
        quads.0.draw(context);
        quads.1.draw(context);
        quads.2.draw(context);
        quads.3.draw(context);

        self.northeast.as_ref().unwrap().draw(context);
        self.northwest.as_ref().unwrap().draw(context);
        self.southwest.as_ref().unwrap().draw(context);
        self.southeast.as_ref().unwrap().draw(context);
      },
      None => {
        self.boundry.draw(context);
      }
    }
  }
}

impl Rect {
  fn draw(&self, canvas: &CanvasRenderingContext2d) {
    canvas.stroke_rect(self.bottom_left.x as f64, (1080. - self.top_right.y) as f64,
    (self.top_right.x - self.bottom_left.x).into(), (self.top_right.y - self.bottom_left.y).into());
  }
}