use std::fs::File;
use std::io::{prelude::*, BufReader};

// The result of trying to get a line from a File.
pub enum FileReadResult {
    Success(String),
    Eof,
    Err
}

// Specialized file-reader for reading "groups" of lines from
// a file. A group is a series of lines separated by newline,
// where all groups are themselves separated by blank lines.
pub struct GroupedFileReader {
    reader: BufReader<File>
}

impl GroupedFileReader {
    pub fn open(path: &str) -> Result<GroupedFileReader, String> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(format!("Could not open file: {}", path))
        };

        let file = GroupedFileReader {
            reader: BufReader::new(file)
        };

        return Ok(file);
    }

    pub fn next(&mut self) -> FileReadResult {
        let mut s = String::new();
        let mut l = String::new();

        loop {
            l.clear();

            match self.reader.read_line(&mut l) {
                // Reached end of file.
                // Return EOF if we didn't collect any lines
                // otherwise, return the lines collected.
                Ok(0) => { // End-of-file.
                    if s.is_empty() {
                        return FileReadResult::Eof;
                    } else {
                        return FileReadResult::Success(s);
                    }
                }
                
                // Normal read. Line will already have been
                // appended to string, so we just need to append
                // the separator character.
                Ok(_) => {
                    // If the line we read is completely whitespace,
                    // that's the end of this group.
                    if l.trim().is_empty() {
                        return FileReadResult::Success(s);
                    } else {
                        s.push_str(&l);
                        s.push('\n');
                    }
                }

                // Error. Return error.
                Err(_) => return FileReadResult::Err
            }
        }
    }
}
