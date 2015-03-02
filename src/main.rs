#![feature(plugin)]
#![plugin(component_store, stainless)]

extern crate ncurses;
extern crate pubsub;
#[macro_use]
extern crate lazy_static;
extern crate uuid;



pub mod events;
pub mod inputs;
pub mod components {
  pub mod position_component;
}
pub mod systems {
  mod curses_input_system;
  mod position_system;
}

use components::position_component::PositionComponent;
use std::collections::HashMap;

component_store!{
  components:
    Position
}

fn main() {
  init();
  render();
  cleanup();
}

fn init() {
  init_ncurses();
}

fn render() {
  render_ncurses();
}

fn cleanup() {
  cleanup_ncurses();
}

fn init_ncurses() {
  ncurses::initscr();
  ncurses::raw();
  ncurses::keypad(ncurses::stdscr, true);
  ncurses::noecho();
}

fn render_ncurses() {
  ncurses::clear();
  ncurses::printw("Sneak POC");
  let _ = ncurses::getch();
}

fn cleanup_ncurses() {
  ncurses::endwin();
}

