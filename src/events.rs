use inputs::GameInput;
use components::position_component::PositionComponent;
use components::dimension_component::DimensionComponent;

#[derive(Hash, PartialEq, Eq, Copy, Debug)]
pub enum EventChannel {
  ChannelGameInput,
  ChannelPositionNew
}

#[derive(PartialEq, Clone, Debug)]
pub enum EventPayload {
  EventKeyboardInput(char),
  EventGameInput(GameInput),

  EventPositionNew(PositionComponent),
  EventDimensionNew(DimensionComponent)
}
