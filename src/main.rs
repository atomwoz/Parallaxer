use std::{thread, time::Duration};

mod ascii_art;
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

fn hide_cursor() {
    let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide);
}

const TERMINAL_WIDTH: usize = 200;
const CANVAS_HEIGHT: usize = 8;

fn main() {
    let lines = ascii_art::load_tunnel();
    let train = ascii_art::load_train();
    let mut i = 0;
    let tunnel_width = ascii_art::get_multiline_width(&lines);
    let train_width = ascii_art::get_multiline_width(&train);
    cls_console();
    hide_cursor();

    //Draw train
    set_cursor_position(0, 1);
    for x in train.iter() {
        println!("{}", x);
    }

    set_cursor_position(0, 0);
    let separator = "-".repeat(TERMINAL_WIDTH);
    println!("{}", separator);
    set_cursor_position(0, (CANVAS_HEIGHT - 1) as u16);
    println!("{}", separator);

    loop {
        let slice_end = i + TERMINAL_WIDTH - train_width;
        let sliced = ascii_art::slice_multiline(&lines, i, slice_end);
        set_cursor_position(train_width as u16, 1);

        //Draw tunnel
        for (i, x) in sliced.iter().enumerate() {
            println!("{}", x);
            set_cursor_position(train_width as u16, i as u16 + 1);
        }
        i += 1;
        if slice_end >= tunnel_width {
            i = 0;
        }
        thread::sleep(Duration::from_millis(16));
    }
}
