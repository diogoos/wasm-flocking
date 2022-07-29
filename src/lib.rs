extern crate console_error_panic_hook;

use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

pub mod quadtree;
use quadtree::QuadTree;
pub mod geometry;
use geometry::{Rect, Point};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}


#[wasm_bindgen]
pub struct FlockingApp {
  canvas: web_sys::HtmlCanvasElement,
  context: CanvasRenderingContext2d,
  qtree: QuadTree,
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
      Point { x: 0, y: 0 },
      Point { x: canvas.width(), y: canvas.height() }
    );

    let mut qtree = QuadTree::new(
      bounds,
      2
    );

    let mut rand = rand::thread_rng();
    for _ in 0..1000 {
      qtree.insert(
        Point {
          x: rand.gen_range(0..canvas.width()),
          y: rand.gen_range(0..canvas.height()),
        }
      );
    }

    FlockingApp {
      qtree,
      canvas,
      context,
    }
  }

  pub fn draw(&self) {
    // draw points
    self.context.set_fill_style(&JsValue::from_str("black"));
    for point in self.qtree.all() {
      self.context.fill_rect(point.x as f64, (self.canvas.height() - point.y) as f64, 5., 5.);
    }

    // draw the clusters
    self.qtree.draw(&self.context);
  }
}

impl QuadTree {
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
    canvas.stroke_rect(self.bottom_left.x as f64, (800 - self.top_right.y) as f64,
    (self.top_right.x - self.bottom_left.x).into(), (self.top_right.y - self.bottom_left.y).into());
  }
}