use ECS;
use pubsub::{Event};
use events::{EventChannel, EventPayload};
use events::EventPayload::*;
type EventVec = Vec<Event<EventChannel, EventPayload>>;

pub struct RotationSystem;

impl RotationSystem {
  fn on_new_rotation(ecs: &mut ECS, payload: EventPayload) -> EventVec {
    let maybe_new_rotation = match payload {
      EventRotationNew(rot) => Some(rot),
      _ => None
    };

    maybe_new_rotation.map(|rot| {
      ecs.rotations.update_rotation(
        rot.eid.clone(), rot
      );
      vec![]
    }).unwrap_or(vec![])
  }
}

#[cfg(test)]
mod tests {
  type RotationSystem = super::RotationSystem;
  describe! creation {
    before_each {
      use ECS;
      use components::rotation_component::RotationComponent;
      use events::EventPayload::*;
      let mut ecs = ECS::new();
      let rot = RotationComponent {
        eid: "rot".to_string(),
        angle: 180.5
      };
      super::RotationSystem::on_new_rotation(&mut ecs, EventRotationNew(rot.clone()));
    }

    it "creates a new rotation" {
      let lookup = ecs.rotations.find_rotation(&"rot".to_string());
      assert!(lookup.eq(&Some(&rot)));
    }
  }
}
