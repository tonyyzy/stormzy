pub struct Node {
    pub id: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Self { id: None }
    }

    pub fn init(&mut self, node_id: String) {
        self.id = Some(node_id);
    }
}
