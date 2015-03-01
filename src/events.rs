use inputs::GameInput;

#[derive(Hash, PartialEq, Eq, Copy, Debug)]
pub enum EventChannel {
  ChannelGameInput
}

#[derive(PartialEq, Eq, Copy, Debug)]
pub enum EventPayload {
  EventKeyboardInput(char),
  EventGameInput(GameInput)
}
