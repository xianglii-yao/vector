use crate::buffer::Buffer;
use crate::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    pub buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if self.buffer.is_empty == true {
                if current_row == height / 3 {
                    Self::draw_welcome_message()?;
                } else {
                    Self::draw_empty_row()?;
                }
            } else {
                if let Some(lines) = &self.buffer.lines {
                    let line_index = self.buffer.scroll_offset + current_row;

                    if line_index < lines.len() {
                        Terminal::print(&lines[line_index])?;
                        Terminal::print("\r\n")?;
                        continue;
                    } else {
                        Self::draw_empty_row()?;
                    }
                }
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message.as_str())?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
