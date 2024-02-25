use std::io::{stdin, stdout, Write};
use std::{time::{self, Duration}, thread};
use std::ptr::write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion;
use rand::Rng;



enum CurrentScreen {
    SplashScreen,
    Leaderboard,
    Game
}

static mut CURRENT_SCREEN: CurrentScreen = CurrentScreen::Game;


fn main() {
    let _stdin = stdin();
    //setting up stdout and going into raw mode
    let _stdout = stdout().into_raw_mode().unwrap();
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
            Key::Char(' ') => start_game(),
            Key::Char('q') => break,
            Key::Char('l') => println!("Show the leaderboard here :--)"),
            _ => (),
        }
    }

}

fn start_game() {
    let mut snake_head = vec![15, 10];
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    write!(stdout, r#"{}{}{}  "#, termion::cursor::Goto(snake_head[0], snake_head[1]), termion::clear::All, termion::color::Bg(termion::color::Cyan))
        .unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            r#"{}{}"#,
            termion::clear::All,
            termion::color::Bg(termion::color::Reset)

        )
            .unwrap();
    }




    sleep(Duration::from_secs(2));
    write!(stdout, r#"{}{}Unpaused"#, termion::cursor::Goto(1, 1), termion::clear::All)
        .unwrap();
    stdout.flush().unwrap();



    // This clears all key presses and prevents program from using old key presses once game is over
    // for _c in stdin.keys() {
    //     match _c.unwrap() {
    //         Key::Char('q') => break,
    //         _ => ()
    //     }
    // }
}

fn random_number(min_num: i16, max_num: i16) -> i16 {
    return rand::thread_rng().gen_range(min_num..max_num + 1);
}

pub fn sleep(dur: Duration) {
    std::io::stdout().flush().unwrap();
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();

    thread::sleep(dur);

    assert!(now.elapsed() >= ten_millis);
}