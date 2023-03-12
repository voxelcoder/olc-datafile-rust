use std::collections::HashMap;

use crate::lexical::Serializable;
use crate::processor::reader::Reader;
use crate::processor::writer::Writer;

/// A datafile is a structured file format that is used to store data. In the words of it's inventor,
/// it is "great for serializing and deserializing data, i.e. Saving Things!".
///
/// The file format itself is very simple. Reading and parsing don't fail unless the file is
/// corrupted. Trying to read data that doesn't conform to that intended type will return a default
/// value, or try to be coerced as best as possible. For example, trying to read an integer from a
/// real will return the integer part of the real.
///
/// A Datafile behaves like a tree, with each node containing a list of other nodes and values.
/// Accessing values or child nodes is done through the `get` method. Should the node not exist,
/// it will be created.
///
/// # Examples
///
/// ```no_run
/// use olc_datafile_rust::Datafile;
///
/// let mut datafile = Datafile::new(None, None);
///
/// let some_node = datafile.get("some_node");
/// some_node.get("name").set_string("Javid", 0);
/// some_node.get("age").set_integer(24, 0);
/// some_node.get("height").set_real(1.88, 0);
///
/// // Save to disk
/// datafile.write("test.txt").unwrap();
///
/// // Load from disk
/// let mut datafile = Datafile::new(None, None);
/// datafile.read("test.txt").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Datafile {
    /// The character to use for separating list values. Defaults to `,`.
    pub list_separator: char,
    /// The character sequence to use for indentation. Defaults to `\t`.
    pub whitespace_sequence: String,

    pub(crate) is_comment: bool,
    pub(crate) contents: Vec<String>,
    pub(crate) object_vec: Vec<(String, Datafile)>,
    pub(crate) object_map: HashMap<String, usize>,
}

const DEFAULT_LIST_SEPARATOR: char = ',';
const DEFAULT_WHITESPACE_SEQUENCE: &str = "\t";

impl Default for Datafile {
    fn default() -> Self {
        Self {
            list_separator: DEFAULT_LIST_SEPARATOR,
            whitespace_sequence: DEFAULT_WHITESPACE_SEQUENCE.to_string(),
            contents: vec![],
            object_vec: vec![],
            object_map: HashMap::new(),
            is_comment: false,
        }
    }
}

impl Datafile {
    /// Creates a new Datafile. The `list_separator` and `whitespace_sequence` arguments are
    /// optional. If not specified, they will default to `,` and `\t` respectively.
    #[must_use]
    pub fn new(list_separator: Option<char>, whitespace_sequence: Option<&str>) -> Self {
        Self {
            list_separator: list_separator.unwrap_or(Self::default().list_separator),
            whitespace_sequence: whitespace_sequence
                .unwrap_or(&Self::default().whitespace_sequence)
                .to_string(),
            ..Default::default()
        }
    }

    /// Writes a datafile to disk.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use olc_datafile_rust::Datafile;
    /// let mut datafile = Datafile::new(None, None);
    /// let some_node = datafile.get("some_node");
    ///
    /// // Save to disk
    /// datafile.write("test.txt").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be written to.
    pub fn write(&self, path: &str) -> std::io::Result<()> {
        let mut writer = Writer::new(self);
        writer.write(path)
    }

    /// Reads a datafile from disk, into the current datafile.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use olc_datafile_rust::Datafile;
    /// let mut datafile = Datafile::new(None, None);
    /// datafile.read("test.txt").unwrap();
    ///
    /// // The Datafile is now populated with the contents of test.txt
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be read from, or is otherwise
    /// corrupted.
    pub fn read(&mut self, path: &str) -> std::io::Result<()> {
        let reader = Reader::new(self);
        reader.read(path)
    }

    /// Sets a string value to the given index. Note that if the index is higher than the current
    /// length of the list, the list will be extended with empty string values.
    pub fn set_string(&mut self, value: &str, index: usize) {
        if index >= self.contents.len() {
            self.contents.resize(index + 1, String::new());
        }

        self.contents[index] = value.to_string();
    }

    /// Gets a string value from the given index. If the index is out of bounds, an empty string
    /// will be returned.
    #[inline]
    #[must_use]
    pub fn get_string(&self, index: usize) -> String {
        self.contents.get(index).unwrap_or(&String::new()).clone()
    }

    /// Appends a real (numeric) value to the datafile.
    #[inline]
    pub fn set_real(&mut self, value: f32, index: usize) {
        self.set_string(&value.serialize(), index);
    }

    /// Gets a real (numeric) value from the given index. If the index is out of bounds, or the
    /// value cannot be parsed as a real, 0.0 will be returned.
    #[inline]
    #[must_use]
    pub fn get_real(&self, index: usize) -> f32 {
        f32::deserialize(&self.get_string(index))
    }

    /// Sets an integer value to the datafile.
    #[inline]
    pub fn set_integer(&mut self, value: i32, index: usize) {
        self.set_string(&value.serialize(), index);
    }

    /// Gets an integer value from the given index. If the index is out of bounds, or the value
    /// cannot be parsed as an integer, 0 will be returned. Real values will be truncated, not
    /// rounded.
    #[inline]
    #[must_use]
    pub fn get_integer(&self, index: usize) -> i32 {
        i32::deserialize(&self.get_string(index))
    }

    /// Returns the number of items in the datafile. Does not include child node's contents.
    #[inline]
    #[must_use]
    pub fn get_value_count(&self) -> usize {
        self.contents.len()
    }

    /// Returns a child node with the given name. If the node does not exist, it will be created.
    /// This can be chained to create a path of nodes. For example, `datafile.get("a").get("b")`
    /// will either return the node `b` under `a`, or create it if it does not exist.
    pub fn get(&mut self, name: &str) -> &mut Self {
        if !self.object_map.contains_key(name) {
            self.object_map
                .insert(name.to_string(), self.object_vec.len());

            self.push_object(
                name,
                Self::new(Some(self.list_separator), Some(&self.whitespace_sequence)),
            );
        }

        &mut self.object_vec[self.object_map[name]].1
    }

    /// Checks if a child node or value with the given name exists.
    #[inline]
    #[must_use]
    pub fn has_property(&self, name: &str) -> bool {
        self.object_map.contains_key(name)
    }

    /// Returns the datafile at a given path using dot notation. If no node exists at the given
    /// path, they will get inserted.
    ///
    /// # Examples
    /// ```no_run
    /// # use olc_datafile_rust::Datafile;
    /// let mut datafile = Datafile::new(None, None);
    /// datafile.read("test.txt").unwrap();
    ///
    /// let node = datafile.get_property("a.b.c");
    /// ```
    pub fn get_property(&mut self, name: &str) -> &Self {
        let Some(dot_index) = name.find('.') else {
            return self.get(name);
        };

        let (node_name, rest) = name.split_at(dot_index);

        if self.has_property(node_name) {
            self.get(node_name).get_property(&rest[1..])
        } else {
            self.get(node_name)
        }
    }

    /// Does the same as `get_property`, but writes it out in array notation.   
    pub fn get_indexed_property(&mut self, name: &str, index: usize) -> &Self {
        self.get_property(&format!("{}[{}]", name, index))
    }

    #[inline]
    pub(crate) fn push_object(&mut self, name: &str, object: Self) {
        self.object_vec.push((name.to_string(), object));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_datafile() -> Datafile {
        Datafile::new(None, None)
    }

    #[test]
    fn test_datafile_basic() {
        let mut datafile = get_datafile();
        assert_eq!(datafile.get_value_count(), 0);

        datafile.set_string("test", 0);
        assert_eq!(datafile.get_value_count(), 1);
        assert_eq!(datafile.get_string(0), "test");

        datafile.set_real(1.5, 1);
        assert_eq!(datafile.get_value_count(), 2);
        assert_eq!(datafile.get_real(1), 1.5);

        datafile.set_integer(1, 2);
        assert_eq!(datafile.get_value_count(), 3);
        assert_eq!(datafile.get_integer(2), 1);
    }

    #[test]
    fn test_datafile_object() {
        let mut datafile = get_datafile();

        let some_node = datafile.get("some_node");
        some_node.get("name").set_string("Javid", 0);
        some_node.get("age").set_integer(24, 0);
        some_node.get("height").set_real(1.88, 0);

        some_node.get("some_list").set_string("item1", 0);
        some_node.get("some_list").set_string("item2", 1);
        some_node.get("some_list").set_string("item3", 2);

        let code = some_node.get("code");
        code.set_string("c++", 0);
        code.set_string("vhdl", 1);
        code.set_string("lua", 2);

        let pc = some_node.get("pc");
        pc.get("processor").set_string("intel", 0);
        pc.get("ram").set_integer(32, 0);

        // PC
        assert_eq!(pc.get("processor").get_value_count(), 1);
        assert_eq!(pc.get("processor").get_string(0), "intel");

        assert_eq!(pc.get("ram").get_value_count(), 1);
        assert_eq!(pc.get("ram").get_integer(0), 32);

        // Some node
        assert_eq!(some_node.get("name").get_value_count(), 1);
        assert_eq!(some_node.get("name").get_string(0), "Javid");

        assert_eq!(some_node.get("age").get_value_count(), 1);
        assert_eq!(some_node.get("age").get_integer(0), 24);

        assert_eq!(some_node.get("height").get_value_count(), 1);
        assert_eq!(some_node.get("height").get_real(0), 1.88);

        // Code
        assert_eq!(some_node.get("code").get_string(0), "c++");
        assert_eq!(some_node.get("code").get_string(1), "vhdl");
        assert_eq!(some_node.get("code").get_string(2), "lua");
    }
}
