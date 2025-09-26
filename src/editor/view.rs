mod buffer;

use std::cmp;

use crate::editor::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
    view::buffer::Buffer,
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    caret_position: Position,
    screen_offset: Position,
}

impl View {
    pub fn from_size(size: Size) -> View {
        View {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: size,
            caret_position: Position::default(),
            screen_offset: Position::default(),
        }
    }

    pub fn get_caret_position(&self) -> Position {
        self.caret_position
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.needs_redraw = true;
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let new_buffer = Buffer::load_file(file_name);
        match new_buffer {
            Ok(buffer) => {
                self.buffer = buffer;
                self.needs_redraw = true;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn evaluate_event(&mut self, command: EditorCommand) -> Result<(), std::io::Error> {
        match command {
            EditorCommand::Move(direction) => self.move_point(direction),
            EditorCommand::Resize(size) => Ok(self.resize(size)),
            _ => panic!("Unexpected editor command"),
        }
    }

    pub fn render(&mut self) -> Result<(), std::io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        let vertical_center = height / 3;
        for row in 0..height {
            if let Some(line) = self.buffer.get_line(row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(row, truncated_line)?;
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::render_line(row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(row, "~")?;
            }
        }

        self.needs_redraw = false;
        Ok(())
    }

    fn move_point(&mut self, direction: Direction) -> Result<(), std::io::Error> {
        let Position { mut x, mut y } = self.caret_position;
        let Size { height, width } = Terminal::size()?;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                y = cmp::min(height.saturating_sub(1), y.saturating_add(1));
            }
            Direction::Left => {
                x = x.saturating_sub(1);
            }
            Direction::Right => {
                x = cmp::min(width.saturating_sub(1), x.saturating_add(1));
            }
            Direction::PageUp => {
                y = 0;
            }
            Direction::PageDown => {
                y = height.saturating_sub(1);
            }
            Direction::Home => {
                x = 0;
            }
            Direction::End => {
                x = width.saturating_sub(1);
            }
        }
        self.caret_position = Position { x, y };
        Ok(())
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::from(" ");
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return String::from("~");
        }

        let padding = (width.saturating_sub(len).saturating_add(1)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        let mut full_message = format!("~{spaces}{welcome_message}");
        full_message.truncate(width);
        full_message
    }

    fn render_line(at: usize, line: &str) -> Result<(), std::io::Error> {
        Terminal::move_caret_to(Position { y: at, x: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line)?;
        Ok(())
    }
}
