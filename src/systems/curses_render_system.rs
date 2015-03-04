use ECS;
use events::EventPayload;
use events::EventChannel::*;
use events::EventPayload::*;
use types::*;
use components::position_component::PositionComponent;
use core::num::ToPrimitive;
use ncurses;

pub struct CursesRenderSystem;

impl CursesRenderSystem {
  pub fn subscribe(pubsub: &mut PubsubECS) {
    pubsub.subscribe(ChannelRender, CursesRenderSystem::on_render);
  }
  fn on_render(ecs: &mut ECS, payload: EventPayload) -> EventVec {
    match payload {
      EventRender => {
        let graphics = ecs.curses_graphics.find_all_curses_graphics();
        for graphic in graphics {
          ecs.positions.find_position(&graphic.eid).map(|pos| {
            let (pos_x, pos_y) = CursesRenderSystem::position_to_curses_coordinates(pos);
            ncurses::mv(pos_y, pos_x);
            ncurses::attron(ncurses::COLOR_PAIR(graphic.color as i16));
            ncurses::printw(&graphic.graphic.to_string()[..]);
            ncurses::attroff(ncurses::COLOR_PAIR(graphic.color as i16));
          });
        }
      }
      _ => ()
    }
    vec![]
  }

  fn position_to_curses_coordinates(pos: &PositionComponent) -> (i32, i32) {
    pos.x.to_i32()
         .and_then(|x| pos.y.to_i32().map(|y| (x/8, y/12)))
         .unwrap_or((0i32, 0i32))
  }
}
