mod editor;
mod logger;
use editor::Editor;

use crate::logger::initialize_logger;

fn main() {
    initialize_logger().expect("Failed to initialize logger");

    Editor::default().run();
}
