use ECS;
use pubsub::Event;
use events::{EventChannel, EventPayload};
use events::EventChannel::*;
use events::EventPayload::*;
use inputs::GameInput::*;
use std::collections::HashMap;
use std::char;

pub struct CursesInputSystem;

lazy_static! {
  static ref INPUT_MAP: HashMap<char, EventPayload> = {
    let mut input_map: HashMap<char, EventPayload> = HashMap::new();
    input_map.insert('q', EventGameInput(InputADir7));
    input_map.insert('Q', EventGameInput(InputBDir7));
    input_map.insert('w', EventGameInput(InputADir8));
    input_map.insert('W', EventGameInput(InputBDir8));
    input_map.insert('e', EventGameInput(InputADir9));
    input_map.insert('E', EventGameInput(InputBDir9));

    input_map.insert('a', EventGameInput(InputADir4));
    input_map.insert('A', EventGameInput(InputBDir4));
    input_map.insert('s', EventGameInput(InputAToggleStance));
    input_map.insert('S', EventGameInput(InputBToggleStance));
    input_map.insert('d', EventGameInput(InputADir6));
    input_map.insert('D', EventGameInput(InputBDir6));

    input_map.insert('z', EventGameInput(InputADir1));
    input_map.insert('Z', EventGameInput(InputBDir1));
    input_map.insert('x', EventGameInput(InputADir2));
    input_map.insert('X', EventGameInput(InputBDir2));
    input_map.insert('c', EventGameInput(InputADir3));
    input_map.insert('C', EventGameInput(InputBDir3));
    input_map
  };
}

impl CursesInputSystem {
  fn on_keypress(_: &mut ECS, payload: EventPayload) -> Vec<Event<EventChannel, EventPayload>> {
    let maybe_keyboard_input = match payload {
      EventKeyboardInput(ch) => Some(ch),
      _ => None
    };

    maybe_keyboard_input.and_then(|ch| INPUT_MAP.get(&ch))
                        .map(|payload| vec![Event { channel: ChannelGameInput, payload: *payload }])
                        .unwrap_or(vec![])
  }
}

#[cfg(test)]
mod tests {
  type CursesInputSystem = super::CursesInputSystem;

  describe! keyboard_input {
    before_each {
      use ECS;
      use pubsub::Event;
      use events::{EventChannel, EventPayload};
      use events::EventChannel::*;
      use events::EventPayload::*;
      use inputs::GameInput::*;
      use super::super::super::CursesInputSystem;

      let mut ecs = ECS::new();
      let input_list = [
        (EventKeyboardInput('q'), EventGameInput(InputADir7)),
        (EventKeyboardInput('Q'), EventGameInput(InputBDir7)),
        (EventKeyboardInput('w'), EventGameInput(InputADir8)),
        (EventKeyboardInput('W'), EventGameInput(InputBDir8)),
        (EventKeyboardInput('e'), EventGameInput(InputADir9)),
        (EventKeyboardInput('E'), EventGameInput(InputBDir9)),

        (EventKeyboardInput('a'), EventGameInput(InputADir4)),
        (EventKeyboardInput('A'), EventGameInput(InputBDir4)),
        (EventKeyboardInput('s'), EventGameInput(InputAToggleStance)),
        (EventKeyboardInput('S'), EventGameInput(InputBToggleStance)),
        (EventKeyboardInput('d'), EventGameInput(InputADir6)),
        (EventKeyboardInput('D'), EventGameInput(InputBDir6)),

        (EventKeyboardInput('z'), EventGameInput(InputADir1)),
        (EventKeyboardInput('Z'), EventGameInput(InputBDir1)),
        (EventKeyboardInput('x'), EventGameInput(InputADir2)),
        (EventKeyboardInput('X'), EventGameInput(InputBDir2)),
        (EventKeyboardInput('c'), EventGameInput(InputADir3)),
        (EventKeyboardInput('C'), EventGameInput(InputBDir3))
      ];
    }

    describe! on_input {
      before_each {
        let expected: Vec<Vec<Event<EventChannel, EventPayload>>> =
          input_list.iter().map(|&(_, output)| vec![Event { channel: ChannelGameInput, payload: output }]).collect();
        let actual: Vec<Vec<Event<EventChannel, EventPayload>>> =
          input_list.iter().map(|&(input, _)| CursesInputSystem::on_keypress(&mut ecs, input)).collect();
      }

      it "has the correct channel" {
        let expected_channels: Vec<EventChannel> = expected.iter().map(|event| event[..].first().unwrap().channel).collect();
        let actual_channels: Vec<EventChannel> = actual.iter().map(|event| event[..].first().unwrap().channel).collect();
        assert_eq!(expected_channels, actual_channels);
      }

      it "has the correct payload" {
        let expected_payloads: Vec<EventPayload> = expected.iter().map(|event| event[..].first().unwrap().payload).collect();
        let actual_payloads: Vec<EventPayload> = actual.iter().map(|event| event[..].first().unwrap().payload).collect();
        assert_eq!(expected_payloads, actual_payloads);
      }
    }
  }
}
