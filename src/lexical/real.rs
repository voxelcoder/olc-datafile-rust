use crate::lexical::Serializable;

impl Serializable<'_> for f32 {
    fn serialize(&self) -> String {
        self.to_string()
    }

    fn deserialize(data: &str) -> Self {
        data.parse::<f32>()
            .unwrap_or_else(|_| data.replace(',', ".").parse::<f32>().unwrap_or(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(0.0.serialize(), "0");
        assert_eq!(1.0.serialize(), "1");
        assert_eq!(1.5.serialize(), "1.5");
        assert_eq!(1.5.serialize(), "1.5");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(f32::deserialize("0"), 0.0);
        assert_eq!(f32::deserialize("1"), 1.0);
        assert_eq!(f32::deserialize("1.5"), 1.5);
        assert_eq!(f32::deserialize("1,5"), 1.5);
    }
}
