use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use crate::datafile::Datafile;

/// A reader for a datafile. This is used to parse a file from disk into a datafile. This is
/// not intended to be used directly, but rather through the `Datafile::read` method. Though
/// usage of this isn't strictly discouraged.
///
/// # Examples
///
/// ```no_run
/// # use olc_datafile_rust::Datafile;
/// let mut datafile = Datafile::new(None, None);
/// datafile.read("path/to/source").unwrap();
/// ```
///
/// # Errors
///
/// This function will return an error if the file cannot be read from.
#[derive(Debug)]
pub struct Reader<'a> {
    top_node: RefCell<&'a mut Datafile>,
}

impl<'a> Reader<'a> {
    /// Creates a new reader for a datafile. Takes a mutable reference to a datafile as an
    /// argument and populates it with the contents of the file.
    pub fn new(datafile: &'a mut Datafile) -> Self {
        Self {
            top_node: RefCell::new(datafile),
        }
    }

    /// Reads a datafile from disk. The top-level datafile should be specified in the structs
    /// constructor. This will overwrite any data that is currently in the datafile.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be opened, or if the file cannot be
    /// read from.
    pub fn read(&self, path: &str) -> std::io::Result<()> {
        let reader = BufReader::new(File::open(path)?);
        let lines = reader.lines().collect();

        Self::read_inner(&mut self.top_node.borrow_mut(), &lines, 0)
    }

    /// Recursively parses a datafile node and it's children.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be read from.
    ///
    /// # Note
    ///
    /// No max depth is specified, so this function will continue to parse until it reaches the
    /// end of the file. This also means that  this function may overflow the stack if the file
    /// is too large. This is not a concern for the intended use of this library,
    /// but it is something to be aware of.
    fn read_inner(
        parent_node: &mut Datafile,
        lines: &Vec<Result<String, Error>>,
        skip: usize,
    ) -> std::io::Result<()> {
        for (i, line) in lines.iter().skip(skip).enumerate() {
            let line_number = i + 1;
            let line = Self::trim_line(line.as_ref(), line_number)?;

            // An empty line or opening brace holds no meaning for the parser. We can skip it.
            if line.is_empty() || line.starts_with('{') {
                continue;
            }

            if line.starts_with('#') {
                let comment_node = Self::construct_comment_node(parent_node.borrow_mut());
                parent_node.push_object(line, comment_node);
                continue;
            }

            // A closing brace means we're done with this node and can safely return to the parent.
            if line.starts_with('}') {
                return Ok(());
            }

            // A line only containing text without any symbols marks a new node.
            if !line.contains('=') {
                let new_node = parent_node.get(line).borrow_mut();
                return Self::read_inner(new_node, lines, line_number + skip);
            }

            let split = line.split_once('=');

            // If there is an equal sign but no value, something went wrong. We just continue.
            if let None | Some((_, "")) = split {
                continue;
            }

            Self::parse_value_from_line(parent_node, split.unwrap());
        }

        Ok(())
    }

    fn parse_value_from_line(parent_node: &mut Datafile, (key, raw_value): (&str, &str)) {
        let mut is_in_quotes = false;
        let mut token_count = 0;
        let mut token = String::new();

        for char in raw_value.chars() {
            // A token is delimited by quotation marks if it contains a list separator.
            // It isn't added to the token itself. When serializing, the writer will handle
            // it's insertion.
            if char == '"' {
                is_in_quotes = !is_in_quotes;
                continue;
            }

            // If we're in quotes, it means that we ignore any list separators, since, as
            // stated above, the delimitation of a token in quotation marks is done to include
            // the list separator in the token itself.
            if is_in_quotes {
                token.push(char);
                continue;
            }

            // A list separator marks the end of a token, and the start of a new one.
            if char == parent_node.list_separator {
                Self::push_token_to_node(key, &token, token_count, parent_node);
                token_count += 1;
                token.clear();
                continue;
            }

            token.push(char);
        }

        if !token.is_empty() {
            Self::push_token_to_node(key, &token, token_count, parent_node);
        }
    }

    #[inline]
    fn push_token_to_node(key: &str, token: &str, index: usize, node: &mut Datafile) {
        let (key, token) = (key.trim(), token.trim());
        node.get(key).set_string(token, index);
    }

    fn construct_comment_node(parent_node: &Datafile) -> Datafile {
        let mut comment_node = Datafile::new(
            Some(parent_node.list_separator),
            Some(&*parent_node.whitespace_sequence),
        );

        comment_node.is_comment = true;
        comment_node
    }

    fn trim_line<'b>(
        line: Result<&'b String, &Error>,
        line_number: usize,
    ) -> Result<&'b str, Error> {
        line.map(|line| line.trim()).map_err(|error| {
            Error::new(
                error.kind(),
                format!("Error reading line {line_number}: {}", error),
            )
        })
    }
}
