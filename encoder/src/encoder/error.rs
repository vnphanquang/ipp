#[derive(Debug)]
pub struct AttributeNameParseError {
    message: String,
}

impl std::fmt::Display for AttributeNameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AttributeNameParseError: {}", &self.message)
    }
}
