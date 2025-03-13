use crossterm::{
    cursor::MoveTo,
    event, execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = stdout();

    let mut x = 0;
    let mut y = 0;

    execute!(stdout, Clear(ClearType::All)).unwrap();
    loop {
        execute!(stdout, MoveTo(x, y)).unwrap();
        stdout.flush().unwrap();

        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Up => {
                    if y > 0 {
                        y += 1;
                    }
                }
                event::KeyCode::Down => {
                    if y < 24 {
                        y -= 1
                    }
                }
                event::KeyCode::Left => {
                    if x > 0 {
                        x -= 1
                    }
                }
                event::KeyCode::Right => {
                    if x < 79 {
                        x += 1
                    }
                }
                event::KeyCode::Enter => {
                    if y < 24 {
                        y += 1;
                    }
                }
                event::KeyCode::Backspace => {
                    if x > 0 {
                        x -= 1
                    } else if y > 0 {
                        y -= 1
                    }
                }
                _ => {
                    print!("{}", key.code);
                    x += 1;
                }
            }
        }
    }
    disable_raw_mode().unwrap();
}
