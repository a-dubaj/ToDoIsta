use termion::{color, style, terminal_size};
use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{}{}",termion::clear::All, termion::cursor::Goto(1,1) );
}
