use crate::buffer::Buffer;
use crate::terminal::{Position, Size, Terminal};
use crate::view::View;
use core::cmp::min;
use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read,
};
use std::io::Error;

pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            location: Location { x: 0, y: 0 },
            view: View {
                buffer: Buffer::read_as_buffer(),
            },
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        if let Some(lines) = self.view.buffer.lines.as_ref() {
            match key_code {
                KeyCode::Left => {
                    if y > 0 {
                        y -= 1;
                    } else if x > 0 {
                        x -= 1;
                        y = lines[x].len(); // Move to end of previous line
                    }
                }
                KeyCode::Right => {
                    if y < lines[x].len() {
                        y += 1;
                    } else if x + 1 < lines.len() {
                        x += 1;
                        y = 0; // Move to beginning of next line
                    }
                }
                KeyCode::Up => {
                    if x > 0 {
                        x -= 1;
                        y = min(y, lines[x].len()); // Keep cursor within valid range
                    } else if self.view.buffer.scroll_offset > 0 {
                        self.view.buffer.scroll_offset -= 1;
                    }
                }
                KeyCode::Down => {
                    if x + 1 < lines.len() {
                        x += 1;
                        y = min(y, lines[x].len()); // Keep cursor within valid range
                    } else if self.view.buffer.scroll_offset + height < lines.len() {
                        self.view.buffer.scroll_offset += 1;
                    }
                }
                KeyCode::PageUp => {
                    y = 0;
                }
                KeyCode::PageDown => {
                    y = height.saturating_sub(1);
                }
                KeyCode::Home => {
                    x = 0;
                }
                KeyCode::End => {
                    x = width.saturating_sub(1);
                }
                _ => Terminal::print("{key_code}").unwrap(),
            }
        }
        self.location = Location { x, y };
        Ok(())
    }
    fn handle_keypress(&mut self, key_code: KeyCode) -> Result<(), Error> {
        match key_code {
            KeyCode::Char(c) => {
                if let Some(lines) = self.view.buffer.lines.as_mut() {
                    if let Some(line) = lines.get_mut(self.location.x) {
                        if self.location.y <= line.len() {
                            line.insert(self.location.y, c);
                            self.location.y += 1;
                        }
                    }
                }
            }
            KeyCode::Enter => {
                self.view.buffer.insert_newline(&self.location);
                self.location.x += 1;
                self.location.y = 0;
            }
            KeyCode::Backspace => {
                if self.location.y > 0 {
                    if let Some(lines) = self.view.buffer.lines.as_mut() {
                        if let Some(line) = lines.get_mut(self.location.x) {
                            line.remove(self.location.y - 1);
                            self.location.y -= 1;
                        }
                    }
                }
            }
            KeyCode::Left => {
                if self.location.y > 0 {
                    self.location.y -= 1;
                }
            }
            KeyCode::Right => {
                if let Some(lines) = self.view.buffer.lines.as_ref() {
                    if let Some(line) = lines.get(self.location.x) {
                        if self.location.y < line.len() {
                            self.location.y += 1;
                        }
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('x') if *modifiers == KeyModifiers::CONTROL => {
                    if self.view.buffer.is_saved == true {
                        self.should_quit = true;
                    }
                }
                KeyCode::Char('s') if *modifiers == KeyModifiers::CONTROL => {
                    Buffer::save_buffer(&self.view.buffer).unwrap();
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => self.handle_keypress(*code)?,
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                row: self.location.x,
                col: self.location.y,
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
