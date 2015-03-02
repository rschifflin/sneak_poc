use ECS;
use pubsub::{Event};
use events::{EventChannel, EventPayload};
use events::EventPayload::*;
type EventVec = Vec<Event<EventChannel, EventPayload>>;

pub struct DimensionSystem;

impl DimensionSystem {
  fn on_new_dimension(ecs: &mut ECS, payload: EventPayload) -> EventVec {
    let maybe_new_dimension = match payload {
      EventDimensionNew(dim) => Some(dim),
      _ => None
    };

    maybe_new_dimension.map(|dim| {
      ecs.dimensions.update_dimension(
        dim.eid.clone(), dim
      );
      vec![]
    }).unwrap_or(vec![])
  }
}

#[cfg(test)]
mod tests {
  type DimensionSystem = super::DimensionSystem;
  describe! creation {
    before_each {
      use ECS;
      use components::dimension_component::DimensionComponent;
      use events::EventPayload::*;
      let mut ecs = ECS::new();
      let dim = DimensionComponent {
        eid: "dim".to_string(),
        width: 0.5,
        length: -2.0,
        height: 0.333
      };
      super::DimensionSystem::on_new_dimension(&mut ecs, EventDimensionNew(dim.clone()));
    }

    it "creates a new dimension" {
      let lookup = ecs.dimensions.find_dimension(&"dim".to_string());
      assert!(lookup.eq(&Some(&dim)));
    }
  }
}
