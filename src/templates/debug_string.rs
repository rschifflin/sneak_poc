use types::*;
use uuid::Uuid;
use components::position_component::PositionComponent;
use components::curses_graphic_component::CursesGraphicComponent;

use pubsub::Event;
use events::EventChannel::*;
use events::EventPayload::*;
use colors::CursesColor::*;

pub fn create(pubsub: &mut PubsubECS, string: &str, xy: (usize, usize)) {
  for (n, ch) in string.chars().enumerate() {
    create_nth_letter(pubsub, ch, n, xy);
  }
}

fn create_nth_letter(pubsub: &mut PubsubECS, ch: char, n: usize, xy: (usize, usize)) {
  let eid = Uuid::new_v4().to_string();
  let (x, y, z) = calculate_pos(n, xy);
  let color = if n % 2 == 0 { CursesColorDefault } else { CursesColorRedWhite };
  let pos = PositionComponent { eid: eid.clone(), x: x, y: y, z: z};
  let graphic = CursesGraphicComponent { eid: eid.clone(), graphic: ch, color: color };
  publish_letter(pubsub, pos, graphic);
}

fn calculate_pos(n: usize, xy: (usize, usize)) -> (f32, f32, f32) {
  let x = ((n + xy.0) * 8) as f32;
  let y = (12 * xy.1) as f32;
  let z = 0f32;
  (x,y,z)
}

fn publish_letter(pubsub: &mut PubsubECS, pos: PositionComponent, graphic: CursesGraphicComponent) {
  pubsub.publish(
    Event {
      channel: ChannelPosition,
      payload: EventPositionNew(pos)
    }
  );

  pubsub.publish(
    Event {
      channel: ChannelCursesGraphic,
      payload: EventCursesGraphicNew(graphic)
    }
  );
}

// TODO: Create Pubsub mock that listens on all channels and answers queries about payloads for testing
