mod editor;
mod logger;
mod terminal;
use editor::Editor;

use crate::logger::initialize_logger;

fn main() {
    initialize_logger().expect("Failed to initialize logger");

    let mut editor = Editor::default();
    editor.run();
}
