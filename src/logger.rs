use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::sync::Mutex;

static LOGGER: Mutex<Option<File>> = Mutex::new(None);

pub fn initialize_logger() -> Result<(), io::Error> {
    let file = File::options().create(true).write(true).open("logs.txt");

    match file {
        Ok(f) => {
            let mut guard = LOGGER.lock().unwrap();
            *guard = Some(f);
        }
        Err(e) => {
            eprintln!("Failed to initialize logger: {}", e);
        }
    }

    if LOGGER.lock().unwrap().is_some() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Logger failed to initialize",
        ))
    }
}

#[allow(dead_code)]
pub fn log<T>(message: T)
where
    T: Display,
{
    let mut logger_guard = LOGGER.lock().unwrap();

    if let Some(ref mut file) = *logger_guard {
        if let Err(e) = writeln!(file, "{}", message) {
            eprintln!("Failed to write to log file: {}", e);
        }
    } else {
        eprintln!("Attempted to log before logger was initialized.");
    }
}
