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

    /// Converts to a little-endian blob.
    ///
    /// Using little-endian is required by sqlite-vec.
    /// See: <https://github.com/asg017/sqlite-vec/blob/04d28bd21773981e2d266bbf6aa4efbd011eb4f6/benchmarks-ann/datasets/cohere1m/build_base_db.py#L20-L22>
    pub fn as_blob(&self) -> Vec<u8> {
        let mut blob = Vec::with_capacity(self.data.len() * 4);
        for v in &self.data {
            blob.extend_from_slice(&v.to_le_bytes());
        }
        blob
    }
}
