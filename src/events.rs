use inputs::GameInput;
use components::position_component::PositionComponent;
use components::dimension_component::DimensionComponent;
use components::rotation_component::RotationComponent;

#[derive(Hash, PartialEq, Eq, Copy, Debug)]
pub enum EventChannel {
  ChannelGameInput,
  ChannelPositionNew,
  ChannelDimensionNew,
  ChannelRotationNew
}

#[derive(PartialEq, Clone, Debug)]
pub enum EventPayload {
  EventKeyboardInput(char),
  EventGameInput(GameInput),

  EventPositionNew(PositionComponent),
  EventDimensionNew(DimensionComponent),
  EventRotationNew(RotationComponent)
}
