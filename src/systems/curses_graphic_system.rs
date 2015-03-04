use ECS;
use pubsub::Event;
use events::{EventChannel, EventPayload};
use events::EventPayload::*;

pub struct CursesGraphicSystem;

impl CursesGraphicSystem {
  fn on_new_curses_graphic(ecs: &mut ECS, payload: EventPayload) -> Vec<Event<EventChannel, EventPayload>> {
    let maybe_curses_graphic = match payload {
      EventCursesGraphicNew(graphic) => Some(graphic),
      _ => None
    };

    maybe_curses_graphic.map(|graphic| {
      ecs.curses_graphics.update_curses_graphic(graphic.eid.clone(), graphic);
      vec![]
    }).unwrap_or(vec![])
  }
}

#[cfg(test)]
mod tests {
  type CursesGraphicSystem = super::CursesGraphicSystem;
  describe! creation {
    before_each {
      use ECS;
      use components::curses_graphic_component::CursesGraphicComponent;
      use colors::CursesColor::*;
      use events::EventPayload::*;
      let mut ecs = ECS::new();
      let graphic = CursesGraphicComponent {
        eid: "graphic".to_string(),
        graphic: 'g',
        color: CursesColorRed
      };
      super::CursesGraphicSystem::on_new_curses_graphic(&mut ecs, EventCursesGraphicNew(graphic.clone()));
    }

    it "creates a new graphic" {
      let lookup = ecs.curses_graphics.find_curses_graphic(&"graphic".to_string());
      assert_eq!(lookup, Some(&graphic));
    }
  }
}
