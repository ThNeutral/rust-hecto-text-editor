use std::{
    fmt::Display,
    io::{self, Write, stdout},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Terminal::clear_screen()?;

        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;

        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        execute!(io::stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(io::stdout(), Clear(ClearType::All))
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(io::stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(io::stdout(), Show)
    }

    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(io::stdout(), MoveTo(position.x, position.y))
    }

    pub fn print<T>(subject: T) -> Result<(), io::Error>
    where
        T: Display,
    {
        queue!(io::stdout(), Print(subject))
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
}
