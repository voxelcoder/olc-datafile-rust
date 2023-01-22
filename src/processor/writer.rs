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
pub struct Writer<'a> {
    pub data_file: &'a Datafile,
    buffer: String,
}

impl<'a> Writer<'a> {
    pub fn new(data_file: &'a Datafile) -> Self {
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
            self.buffer.replace_range(0..1, "");
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
    fn write_node(&mut self, datafile: &'a Datafile, mut indent_level: usize) {
        for (name, node) in datafile.object_vec.iter() {
            if node.object_vec.is_empty() {
                self.write_value_key(node, name, indent_level);
                self.write_list(node);
                self.buffer.push('\n');
                continue;
            }

            self.write_node_header(node, indent_level, name);

            indent_level += 1;
            self.write_node(node, indent_level);
            indent_level = usize::max(0, indent_level - 1);

            self.write_node_footer(node, indent_level);
        }
    }

    /// Takes a node's contents and writes it to the buffer in list format.
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
    fn write_list(&mut self, node: &Datafile) {
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
    fn write_node_header(&mut self, node: &Datafile, indent_level: usize, name: &str) {
        let indentation = Self::get_indentation(indent_level, &node.whitespace_sequence);
        self.buffer
            .push_str(&format!("\n{indentation}{}\n{indentation}{{\n", name,));
    }

    /// Writes a node's footer to the buffer. This is just the closing brace.
    #[inline]
    fn write_node_footer(&mut self, node: &Datafile, indent_level: usize) {
        self.buffer.push_str(&format!(
            "{}}}\n",
            Self::get_indentation(indent_level, &node.whitespace_sequence)
        ));
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
    fn write_value_key(&mut self, node: &Datafile, name: &str, indent_level: usize) {
        self.buffer.push_str(&format!(
            "{}{name} {} ",
            Self::get_indentation(indent_level, &node.whitespace_sequence),
            if node.is_comment { "" } else { "=" }
        ));
    }

    /// Returns a string of indentation based on the node's indentation level.  
    #[inline]
    fn get_indentation(indent_level: usize, indentation: &str) -> String {
        (0..indent_level).map(|_| indentation).collect::<String>()
    }
}
