extern crate termion;

use termion::terminal_size;
use termion::color;

fn main() {
    let (width, _) = terminal_size().unwrap();
    let line_bar = "█".repeat(width as usize);

    let flag_blue = color::Fg(color::LightBlue);
    let flag_pink = "\x1B[38;5;219m";
    let flag_white = color::Fg(color::White);
    let reset_style = color::Fg(color::Reset);

    println!("{}{}{}", flag_blue, line_bar, reset_style);
    println!("{}{}{}", flag_pink, line_bar, reset_style);
    println!("{}{}{}", flag_white, line_bar, reset_style);
    println!("{}{}{}", flag_pink, line_bar, reset_style);
    println!("{}{}{}", flag_blue, line_bar, reset_style);
}
