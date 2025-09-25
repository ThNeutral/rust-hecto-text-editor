pub struct Buffer {
    lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            lines: vec![
                String::from("Hello, World!"),
                String::from("Hello, World2!"),
                String::from(""),
            ],
        }
    }
}

impl Buffer {
    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}
