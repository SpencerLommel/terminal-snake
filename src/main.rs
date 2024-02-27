use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::ptr::write;
use std::{
    thread,
    time::{self, Duration},
};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

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
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Initial game setup
    let mut game_running = true;
    let mut snake_head = vec![15, 10];
    let mut direction = Direction::Right;
    let mut prev_head = snake_head.clone(); // For clearing previous position

    write!(
        stdout,
        "{}{}  ",
        cursor::Goto(snake_head[0], snake_head[1]),
        color::Bg(color::Cyan)
    )
    .unwrap();
    stdout.flush().unwrap();

    while game_running {
        let stdin = stdin();
        if let Some(key) = stdin.keys().next() {
            if let Ok(key) = key {
                match key {
                    Key::Char('q') => break,
                    Key::Char('w') if direction != Direction::Down => direction = Direction::Up,
                    Key::Char('s') if direction != Direction::Up => direction = Direction::Down,
                    Key::Char('a') if direction != Direction::Right => direction = Direction::Left,
                    Key::Char('d') if direction != Direction::Left => direction = Direction::Right,
                    _ => (),
                }
            }
        }

        // Update snake position (include collision checks later)
        match direction {
            Direction::Up => snake_head[1] -= 1,
            Direction::Down => snake_head[1] += 1,
            Direction::Left => snake_head[0] -= 2,
            Direction::Right => snake_head[0] += 2,
        }

        // Clear the previous snake position
        write!(
            stdout,
            "{}{}",
            cursor::Goto(prev_head[0], prev_head[1]),
            color::Bg(color::Reset)
        )
        .unwrap();

        // Draw new snake head
        write!(
            stdout,
            "{}{}{}  ",
            cursor::Goto(snake_head[0], snake_head[1]),
            clear::CurrentLine,
            color::Bg(color::Cyan)
        )
        .unwrap();

        prev_head = snake_head.clone(); // Update for next frame

        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }

    // Restore terminal
    write!(stdout, "{}{}", clear::All, color::Bg(color::Reset)).unwrap();
    stdout.flush().unwrap();
}

// Helper enum for directions
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
