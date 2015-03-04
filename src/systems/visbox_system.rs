use ECS;
use events::EventPayload;
use events::EventPayload::*;
use types::*;

pub struct VisboxSystem;

impl VisboxSystem {
  fn on_new_visbox(ecs: &mut ECS, payload: EventPayload) -> EventVec {
    let maybe_new_visbox = match payload {
      EventVisboxNew(vis) => Some(vis),
      _ => None
    };

    maybe_new_visbox.map(|vis| {
      ecs.visboxes.update_visbox(
        vis.eid.clone(), vis
      );
      vec![]
    }).unwrap_or(vec![])
  }
}

#[cfg(test)]
mod tests {
  type VisboxSystem = super::VisboxSystem;
  describe! creation {
    before_each {
      use ECS;
      use components::visbox_component::VisboxComponent;
      use util::rect::Rect;
      use visibility::Visibility::*;
      use events::EventPayload::*;
      let mut ecs = ECS::new();
      let rect = Rect::new(0.0, 0.0, 0.0, 0.0);
      let vis = VisboxComponent {
        eid: "vis".to_string(),
        rect: rect,
        vis: Opaque
      };
      super::VisboxSystem::on_new_visbox(&mut ecs, EventVisboxNew(vis.clone()));
    }

    it "creates a new visbox" {
      let lookup = ecs.visboxes.find_visbox(&"vis".to_string());
      assert!(lookup.eq(&Some(&vis)));
    }
  }
}
