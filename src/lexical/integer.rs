use crate::lexical::Serializable;

impl Serializable<'_> for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }

    fn deserialize(data: &str) -> Self {
        data.parse::<i32>()
            // If a float is passed, we just truncate it.
            .unwrap_or_else(|_| data.replace(',', ".").parse::<f32>().unwrap_or(0.0) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(0.serialize(), "0");
        assert_eq!(1.serialize(), "1");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(i32::deserialize("0"), 0);
        assert_eq!(i32::deserialize("1"), 1);
        assert_eq!(i32::deserialize("1.5"), 1);
        assert_eq!(i32::deserialize("1,5"), 1);
    }
}
