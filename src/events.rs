use inputs::GameInput;
use components::position_component::PositionComponent;
use components::dimension_component::DimensionComponent;
use components::rotation_component::RotationComponent;
use components::curses_graphic_component::CursesGraphicComponent;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum EventChannel {
  ChannelTick,
  ChannelRender,

  ChannelGameInput,
  ChannelPosition,
  ChannelDimension,
  ChannelRotation,
  ChannelCursesGraphic
}

#[derive(PartialEq, Clone, Debug)]
pub enum EventPayload {
  EventTick,
  EventRender,

  EventKeyboardInput(char),
  EventGameInput(GameInput),

  EventPositionNew(PositionComponent),
  EventDimensionNew(DimensionComponent),
  EventRotationNew(RotationComponent),
  EventCursesGraphicNew(CursesGraphicComponent)
}
