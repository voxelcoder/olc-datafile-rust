use crate::lexical::Serializable;

impl Serializable<'_> for String {
    fn serialize(&self) -> String {
        self.clone()
    }

    fn deserialize(data: &str) -> Self {
        data.to_owned()
    }
}

impl<'a> Serializable<'a> for &'a str {
    fn serialize(&self) -> String {
        (*self).to_string()
    }

    fn deserialize(data: &'a str) -> Self {
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!("".serialize(), "");
        assert_eq!("Hello, world!".serialize(), "Hello, world!");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(String::deserialize(""), "");
        assert_eq!(String::deserialize("Hello, world!"), "Hello, world!");
    }
}
