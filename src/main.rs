use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion;

fn main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    show_splash_screen();
}


fn show_splash_screen() {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, r#"{}{}Welcome to Spencers Rust Snake Game! Press 'space' to start, 'q' to quit, 'l' for leaderboard."#, termion::cursor::Goto(1, 1), termion::clear::All)
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char(' ') => println!("Space was pressed!"),
            Key::Char('q') => break,
            Key::Char('l') => println!("Show the leaderboard here :--)"),
            _ => (),
        }
    }

}

fn start_game() {
    termion::
}