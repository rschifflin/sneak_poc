use ECS;
use events::EventPayload;
use events::EventChannel::*;
use events::EventPayload::*;
use types::*;
use components::position_component::PositionComponent;
use core::num::ToPrimitive;

use ncurses;
use helpers::curses_helpers;

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
            curses_helpers::mv_world_coords(pos.x, pos.y);
            curses_helpers::print_graphic(&graphic);
          });
        }
      }
      _ => ()
    }
    vec![]
  }
}
