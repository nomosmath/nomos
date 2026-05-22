pub struct NomosClient {
    endpoint: String,
}

impl NomosClient {
    pub fn new(endpoint: &str) -> Self {
        Self { endpoint: endpoint.to_string() }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}
