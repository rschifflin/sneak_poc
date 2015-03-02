use uuid::Uuid;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct PositionComponent {
  pub eid: String,
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl PositionComponent {
  pub fn new(x: f32, y: f32, z: f32) -> PositionComponent {
    PositionComponent {
      eid: Uuid::new_v4().to_string(),
      x: x,
      y: y,
      z: z
    }
  }
}
