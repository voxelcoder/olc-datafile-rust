use std::fs::File;
use std::io::Write;

use crate::datafile::Datafile;

/// A writer for a datafile. This is used to write a datafile to a file. This is
/// not intended to be used directly, but rather through the `Datafile::write`
/// method. Though usage of this isn't strictly discouraged.
///
/// # Examples
///
/// ```no_run
/// # use olc_datafile_rust::Datafile;
/// let mut datafile = Datafile::new(None, None);
/// let some_node = datafile.get("some_node");
///
/// some_node.get("name").set_string("Javid", 0);
///
/// datafile.write("path/to/destination").unwrap();
/// ```
#[derive(Debug)]
pub struct Writer<'a> {
    pub data_file: &'a Datafile,
    buffer: String,
}

impl<'a> Writer<'a> {
    #[must_use]
    pub const fn new(data_file: &'a Datafile) -> Self {
        Self {
            data_file,
            buffer: String::new(),
        }
    }

    /// Writes a datafile to disk. The top-level datafile should be specified in the structs
    /// constructor.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be written to.
    pub fn write(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        self.write_node(self.data_file, 0);

        // Deviation from the original implementation. I just like this better. Removes the leading
        // newline at the top of the file.
        if self.buffer.starts_with('\n') {
            self.buffer.remove(0);
        }

        file.write_all(self.buffer.as_bytes())
    }

    /// Writes a node to the file. Should the node itself contain other nodes, it will recursively
    /// call itself to write those nodes.
    ///
    /// # Arguments
    ///
    /// * `node` - datafile (node) to write
    /// * `indent` - the number of indentations to write before the node
    fn write_node(&mut self, datafile: &'a Datafile, indent_level: usize) {
        for (name, node) in &datafile.object_vec {
            if node.object_vec.is_empty() {
                self.write_key(node, name, indent_level);
                self.write_value(node);
                continue;
            }

            self.write_node_header(indent_level, name);
            self.write_node(node, indent_level + 1);
            self.write_node_footer(indent_level);
        }
    }

    /// Writes a node's key to the buffer. If the node has a value, it will be followed by an
    /// equal sign. If the node is a comment, it will be written as a comment.
    ///
    /// # Example
    ///
    /// A node with a value:
    ///
    /// ```text
    /// "foo = "
    /// ```
    ///
    /// A node with a comment:
    ///
    /// ```txt
    /// "#foo"
    /// ```
    #[inline]
    fn write_key(&mut self, node: &Datafile, name: &str, indent_level: usize) {
        self.buffer.push_str(&format!(
            "{}{name}{}",
            self.get_indentation(indent_level),
            if node.is_comment { "" } else { " = " },
        ));
    }

    /// Takes a node's content and writes it to the buffer in list format.
    ///
    /// # Example
    /// (The following examples assume a list separator of `,` specified in the datafile.)
    ///
    ///
    /// ```no_run
    /// let contents = ["foo", "bar", "baz"];
    /// ```
    /// Gets written to the buffer as:
    /// ```text
    /// "foo, bar, baz"
    /// ```
    ///
    /// If a value contains a list separator, it will be delimited by quotes.
    /// ```no_run
    /// let contents = ["foo", "bar, baz"];
    /// ```
    ///
    /// Gets written to the buffer as:
    ///
    /// ```text
    /// ""foo, "bar, baz""
    /// ```
    #[inline]
    fn write_value(&mut self, node: &Datafile) {
        self.buffer.push_str(
            &node
                .contents
                .iter()
                .map(|value| {
                    if value.contains(self.data_file.list_separator) {
                        format!("\"{}\"", value)
                    } else {
                        value.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(&format!("{} ", self.data_file.list_separator)),
        );

        self.buffer.push('\n');
    }

    /// Writes a node's header to the buffer.
    ///
    /// # Example
    ///
    /// A header for node "foo" will be written to the buffer as:
    /// ```txt
    /// foo
    /// {
    /// ```
    #[inline]
    fn write_node_header(&mut self, indent_level: usize, name: &str) {
        let indentation = self.get_indentation(indent_level);
        self.buffer
            .push_str(&format!("\n{indentation}{}\n{indentation}{{\n", name));
    }

    /// Writes a node's footer to the buffer. This is just the closing brace.
    #[inline]
    fn write_node_footer(&mut self, indent_level: usize) {
        self.buffer
            .push_str(&format!("{}}}\n", self.get_indentation(indent_level)));
    }

    /// Returns a string of indentation based on the node's indentation level.  
    #[inline]
    fn get_indentation(&self, indent_level: usize) -> String {
        self.data_file.whitespace_sequence.repeat(indent_level)
    }
}
