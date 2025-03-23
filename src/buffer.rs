use std::fs::{read_to_string, write};

use crate::editor::Location;

pub struct Buffer {
    pub lines: Option<Vec<String>>,
    pub is_empty: bool,
    pub scroll_offset: usize,
    pub is_saved: bool,
    path: Option<String>,
}

impl Buffer {
    pub fn read_as_buffer() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut buffer_data: Vec<String> = Vec::new();
        if let Some(path) = args.get(1) {
            let content = read_to_string(path).unwrap();
            for line in content.lines() {
                buffer_data.push(line.to_string());
            }
            Buffer {
                lines: Some(buffer_data),
                is_empty: false,
                scroll_offset: 0,
                is_saved: true,
                path: Some(path.to_string()),
            }
        } else {
            Buffer {
                lines: None,
                is_empty: true,
                scroll_offset: 0,
                is_saved: true,
                path: None,
            }
        }
    }
    // pub fn insert(mut self, at: &Location, c: char) {
    //     if let Some(lines) = self.lines.as_mut() {
    //         if at.y > lines.len() {
    //             return;
    //         }
    //         if c == '\n' {
    //             self.insert_newline(at);
    //         } else if at.y == lines.len() {
    //             let mut row = String::new();
    //             row.insert(0, c);
    //             lines.push(row);
    //         } else {
    //             #[allow(clippy::indexing_slicing)]
    //             let row = &mut lines[at.y];
    //             row.insert(at.x, c);
    //         }
    //     }
    // }
    pub fn insert_newline(&mut self, at: &Location) {
        if let Some(rows) = self.lines.as_mut() {
            if at.y >= rows.len() {
                return;
            }

            if at.y == rows.len() - 1 {
                rows.push(String::new());
                return;
            }

            if let Some(current_row) = rows.get_mut(at.y) {
                let new_row = current_row.split_off(at.x);
                rows.insert(at.y + 1, new_row);
            }
        }
    }
    pub fn save_buffer(&self) -> Result<(), std::io::Error> {
        if let Some(path) = &self.path {
            if let Some(contents) = &self.lines {
                write(path, contents.join("\n")).unwrap();
            }
        }
        Ok(())
    }
}
