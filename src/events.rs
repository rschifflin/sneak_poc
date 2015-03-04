use inputs::GameInput;
use components::position_component::PositionComponent;
use components::visbox_component::VisboxComponent;
use components::rotation_component::RotationComponent;
use components::curses_graphic_component::CursesGraphicComponent;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum EventChannel {
  ChannelTick,
  ChannelRender,

  ChannelGameInput,
  ChannelPosition,
  ChannelVisbox,
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
  EventVisboxNew(VisboxComponent),
  EventRotationNew(RotationComponent),
  EventCursesGraphicNew(CursesGraphicComponent)
}
