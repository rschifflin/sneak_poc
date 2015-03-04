#![feature(core, plugin)]
#![plugin(component_store, stainless)]

extern crate ncurses;
extern crate pubsub;
#[macro_use]
extern crate lazy_static;
extern crate uuid;
extern crate core;

pub mod types;
pub mod colors;
pub mod events;
pub mod inputs;
pub mod visibility;

pub mod util {
  pub mod rect;
}

pub mod helpers {
  pub mod curses_helpers;
}
pub mod components {
  pub mod position_component;
  pub mod visbox_component;
  pub mod rotation_component;
  pub mod curses_graphic_component;
}
pub mod systems {
  pub mod curses_input_system;
  pub mod curses_graphic_system;
  pub mod curses_render_system;
  pub mod position_system;
  pub mod visbox_system;
  pub mod rotation_system;
}
pub mod templates {
  pub mod debug_string;
}

use components::position_component::PositionComponent;
use components::visbox_component::VisboxComponent;
use components::rotation_component::RotationComponent;
use components::curses_graphic_component::CursesGraphicComponent;

use systems::position_system::PositionSystem;
use systems::curses_graphic_system::CursesGraphicSystem;
use systems::curses_render_system::CursesRenderSystem;

use templates::debug_string;

use pubsub::{Pubsub, Event};
use events::EventChannel::*;
use events::EventPayload::*;
use types::PubsubECS;

use std::collections::HashMap;

component_store!{
  components:
    Position
    Visbox/Visboxes
    Rotation
    CursesGraphic
}

fn main() {
  let mut ecs = ECS::new();
  let mut pubsub: PubsubECS = Pubsub::new(&mut ecs);

  init(&mut pubsub);
  build_world(&mut pubsub);
  game_loop(&mut pubsub);
  cleanup();
}

fn init(pubsub: &mut PubsubECS) {
  init_systems(pubsub);
  init_ncurses();
}

fn init_systems(pubsub: &mut PubsubECS) {
  PositionSystem::subscribe(pubsub);
  CursesGraphicSystem::subscribe(pubsub);
  CursesRenderSystem::subscribe(pubsub);
}

fn init_ncurses() {
  ncurses::initscr();
  ncurses::start_color();
  ncurses::init_pair(1, ncurses::COLOR_RED, ncurses::COLOR_BLACK);
  ncurses::raw();
  ncurses::keypad(ncurses::stdscr, true);
  ncurses::noecho();
}

fn build_world(pubsub: &mut PubsubECS) {
  debug_string::create(pubsub, "Sneak POC", (2,1));
}

fn game_loop(pubsub: &mut PubsubECS) {
  loop {
    let ch = ncurses::getch();
    if ch == ncurses::KEY_F1 { break; }
    update(pubsub);
    render(pubsub);
  }
}

fn update(pubsub: &mut PubsubECS) {
    pubsub.publish(
      Event {
        channel: ChannelTick,
        payload: EventTick
      }
    )
}

fn render(pubsub: &mut PubsubECS) {
  pubsub.publish(
    Event {
      channel: ChannelRender,
      payload: EventRender
    }
  )
}

fn cleanup() {
  cleanup_ncurses();
}

fn cleanup_ncurses() {
  ncurses::endwin();
}
