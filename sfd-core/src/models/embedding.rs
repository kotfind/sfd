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

    /// Converts to a big-endian blob.
    pub fn as_blob(&self) -> Vec<u8> {
        let mut blob = Vec::with_capacity(self.data.len() * 4);
        for v in &self.data {
            blob.extend_from_slice(&v.to_be_bytes());
        }
        blob
    }
}
