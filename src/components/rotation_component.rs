use uuid::Uuid;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct RotationComponent {
  pub eid: String,
  pub angle: f32,
}

impl RotationComponent {
  pub fn new(angle: f32) -> RotationComponent {
    RotationComponent {
      eid: Uuid::new_v4().to_string(),
      angle: angle
    }
  }

  pub fn degrees(&self) -> f32 { self.angle }
}
