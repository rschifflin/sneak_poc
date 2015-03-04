use core::num::ToPrimitive;
use components::curses_graphic_component::CursesGraphicComponent;
use ncurses;

pub fn print_graphic(graphic: &CursesGraphicComponent) {
  ncurses::attron(ncurses::COLOR_PAIR(graphic.color as i16));
  ncurses::printw(&graphic.graphic.to_string()[..]);
  ncurses::attroff(ncurses::COLOR_PAIR(graphic.color as i16));
}

pub fn mv_world_coords(x: f32, y: f32) {
  x.to_i32()
   .and_then(|x| y.to_i32()
   .and_then(|y| Some((x/8, y/12)))
   .map(|(ix,iy)| ncurses::mv(iy, ix)));
}
