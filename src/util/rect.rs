#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Rect {
  x1: f32,
  x2: f32,
  y1: f32,
  y2: f32
}

impl Rect {
  pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
    Rect {
      x1: x1,
      x2: x2,
      y1: y1,
      y2: y2
    }
  }
}
