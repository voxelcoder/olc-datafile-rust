mod integer;
mod real;
mod string;

pub(crate) trait Serializable<'a> {
    fn serialize(&self) -> String;

    fn deserialize(data: &'a str) -> Self
    where
        Self: Sized;
}
