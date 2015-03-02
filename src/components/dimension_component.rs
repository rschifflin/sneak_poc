use uuid::Uuid;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct DimensionComponent {
  pub eid: String,
  pub width: f32,
  pub length: f32,
  pub height: f32
}

impl DimensionComponent {
  pub fn new(width: f32, length: f32, height: f32) -> DimensionComponent {
    DimensionComponent {
      eid: Uuid::new_v4().to_string(),
      width: width,
      length: length,
      height: height
    }
  }
}
