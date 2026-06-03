#[derive(Debug, Clone)]
pub struct Ident {
    name: String,
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
