pub trait PrimitiveOrOwnedStringToOwned {
    fn convert(self) -> String;
}

impl PrimitiveOrOwnedStringToOwned for &str {
    fn convert(self) -> String {
        self.to_string()
    }
}

impl PrimitiveOrOwnedStringToOwned for String {
    fn convert(self) -> String {
        self
    }
}