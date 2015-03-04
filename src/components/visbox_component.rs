use uuid::Uuid;
use util::rect::Rect;
use visibility::Visibility;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct VisboxComponent {
  pub eid: String,
  pub rect: Rect,
  pub vis: Visibility
}

impl VisboxComponent {
  pub fn new(rect: Rect, vis: Visibility) -> VisboxComponent {
    VisboxComponent {
      eid: Uuid::new_v4().to_string(),
      rect: rect,
      vis: vis
    }
  }
}
