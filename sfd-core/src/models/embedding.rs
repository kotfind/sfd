/// An embedding vector.
#[derive(Debug, Clone)]
pub struct Embedding {
    data: Vec<f32>,
}

impl Embedding {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }
}
