use std::fs;

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn load_file(file_name: &str) -> Result<Self, std::io::Error> {
        let file_contents = fs::read_to_string(file_name)?;

        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(String::from(line));
        }

        Ok(Buffer { lines })
    }

    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
