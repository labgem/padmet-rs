#[derive(Debug, Clone)]
pub struct Policy;

impl Default for Policy {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy {
    pub fn new() -> Self {
        Policy {}
    }
}
