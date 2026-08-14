/// Transaction management
/// Implements: AC §45

pub struct Transaction {
    pub generation: u64,
}

impl Transaction {
    pub fn new(generation: u64) -> Self {
        Self { generation }
    }
}
