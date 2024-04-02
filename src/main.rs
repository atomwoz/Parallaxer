use std::{iter, thread, time::Duration};

mod ascii_art;
mod data;

fn cls_console() {
    if cfg!(windows) {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    } else {
        print!("{}[2J", 27 as char);
    }
}

fn set_cursor_position(x: u16, y: u16) {
    let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y));
}

const TERMINAL_WIDTH: usize = 80;

fn main() {
    let lines: Vec<&str> = data::TUNNEL.lines().collect();
    let mut i = 0;
    cls_console();
    loop {
        let sliced = ascii_art::slice_multiline(&lines, i, i + TERMINAL_WIDTH);
        set_cursor_position(0, 0);
        for x in sliced {
            println!("{}", x);
        }
        i += 1;
        if (i + TERMINAL_WIDTH >= ascii_art::get_multiline_width(&lines)) {
            i = 0;
        }
        thread::sleep(Duration::from_millis(20));
    }
}
