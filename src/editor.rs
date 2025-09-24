use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers, read,
};

use crate::terminal::{Position, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().expect("Failed to initialize editor");

        let result = self.repl();
        Terminal::terminate().expect("Failed to terminate editor");
        result.expect("REPL error")
    }

    fn initialize() -> Result<(), std::io::Error> {
        Terminal::initialize()?;

        Self::draw_rows()?;
        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;

        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        for i in 0..size.height {
            Terminal::move_cursor_to(Position { x: 0, y: i })?;
            Terminal::clear_line()?;

            if i == size.height / 2 {
                Self::draw_welcome_message_line()?;
                continue;
            }

            Self::draw_empty_line()?;
        }

        Ok(())
    }

    fn draw_welcome_message_line() -> Result<(), std::io::Error> {
        let welcome_message = format!("{} editor -- version {}", NAME, VERSION);

        let message_size = welcome_message.chars().count() as u16;
        let terminal_size = Terminal::size()?.width;

        if terminal_size < message_size {
            return Ok(());
        }

        let padding = " ".repeat((terminal_size - message_size) as usize / 2);
        let welcome_line = format!("~{}{}", padding, welcome_message);

        Terminal::print(welcome_line)?;

        Ok(())
    }

    fn draw_empty_line() -> Result<(), std::io::Error> {
        Terminal::print("~")?;

        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        while !self.should_quit {
            let event = read()?;
            self.evaluate_event(&event)?;
            self.refresh_screen()?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }

        if let Event::Resize(_, _) = event {
            Self::draw_rows()?;
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }
}
