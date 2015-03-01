extern crate ncurses;

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

