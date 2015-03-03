use inputs::GameInput;
use components::position_component::PositionComponent;
use components::dimension_component::DimensionComponent;
use components::rotation_component::RotationComponent;
use components::curses_graphic_component::CursesGraphicComponent;

#[derive(Hash, PartialEq, Eq, Copy, Debug)]
pub enum EventChannel {
  ChannelGameInput,
  ChannelPosition,
  ChannelDimension,
  ChannelRotation,
  ChannelCursesGraphic
}

#[derive(PartialEq, Clone, Debug)]
pub enum EventPayload {
  EventRender,

  EventKeyboardInput(char),
  EventGameInput(GameInput),

  EventPositionNew(PositionComponent),
  EventDimensionNew(DimensionComponent),
  EventRotationNew(RotationComponent),
  EventCursesGraphicNew(CursesGraphicComponent)
}
