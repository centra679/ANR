pub struct MockHAL {
    initialized: bool,
}

impl MockHAL {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn initialize(&mut self) -> crate::Result<()> {
        self.initialized = true;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn shutdown(&mut self) {
        self.initialized = false;
    }
}

impl Default for MockHAL {
    fn default() -> Self {
        Self::new()
    }
}

impl super::Hal for MockHAL {
    fn initialize(&mut self) -> crate::Result<()> {
        self.initialized = true;
        Ok(())
    }

    fn is_ready(&self) -> bool {
        self.initialized
    }

    fn shutdown(&mut self) {
        self.initialized = false;
    }
}
