use crossterm::event::{
    Event::{self},
    KeyEvent, KeyEventKind, read,
};
use std::{
    env,
    io::{Error, ErrorKind},
    panic::{set_hook, take_hook},
};

mod editorcommand;
mod terminal;
mod view;
use terminal::{Position, Terminal};

use crate::editor::{editorcommand::EditorCommand, view::View};

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Drop for Editor {
    fn drop(&mut self) {
        unimplemented!();
    }
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;

        let size = Terminal::size()?;
        let mut view = View::from_size(size);

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name)?;
        }

        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read().unwrap_or_else(|err| panic!("Failed to read event: {}", err));
            self.evaluate_event(event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: Event) -> Result<(), Error> {
        let should_evaluate = match &event {
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                ..
            }) => true,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_evaluate {
            return Ok(());
        }

        match EditorCommand::try_from(event) {
            Ok(command) => match command {
                EditorCommand::Quit => {
                    self.should_quit = true;
                    Ok(())
                }
                _ => self.view.evaluate_event(command),
            },
            Err(err) => Err(std::io::Error::new(ErrorKind::Other, err)),
        }
    }
    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        self.view.render()?;
        Terminal::move_caret_to(self.view.get_caret_position())?;

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
