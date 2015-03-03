use uuid::Uuid;
use colors::CursesColor;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct CursesGraphicComponent {
  pub eid: String,
  pub graphic: char,
  pub color: CursesColor
}

impl CursesGraphicComponent {
  pub fn new(graphic: char, color: CursesColor) -> CursesGraphicComponent {
    CursesGraphicComponent {
      eid: Uuid::new_v4().to_string(),
      graphic: graphic,
      color: color
    }
  }
}
