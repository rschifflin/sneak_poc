use ECS;
use events::EventPayload;
use events::EventChannel::*;
use events::EventPayload::*;
use types::*;

pub struct PositionSystem;

impl PositionSystem {
  pub fn subscribe(pubsub: &mut PubsubECS) {
    pubsub.subscribe(ChannelPosition, PositionSystem::on_new_position);
  }

  fn on_new_position(ecs: &mut ECS, payload: EventPayload) -> EventVec {
    let maybe_new_position = match payload {
      EventPositionNew(pos) => Some(pos),
      _ => None
    };

    maybe_new_position.map(|pos| {
      ecs.positions.update_position(
        pos.eid.clone(), pos
      );
      vec![]
    }).unwrap_or(vec![])
  }
}

#[cfg(test)]
mod tests {
  type PositionSystem = super::PositionSystem;
  describe! creation {
    before_each {
      use ECS;
      use components::position_component::PositionComponent;
      use events::EventPayload::*;
      let mut ecs = ECS::new();
      let pos = PositionComponent {
        eid: "pos".to_string(),
        x: 0.5,
        y: -2.0,
        z: 0.333
      };
      super::PositionSystem::on_new_position(&mut ecs, EventPositionNew(pos.clone()));
    }

    it "creates a new position" {
      let lookup = ecs.positions.find_position(&"pos".to_string());
      assert!(lookup.eq(&Some(&pos)));
    }
  }
}
