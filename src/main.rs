use termion::{color, style, terminal_size};
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use ncurses::*;

fn main() {
    initscr();
    addstr("Hello!");
    refresh();

    let mut quit = false;
    while !quit {
        let key = getch();
        match key as u8 as char {
           'q' => quit = true,
            _=> {}
        }
    }
    endwin();
}