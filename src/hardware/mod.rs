pub mod mock;

pub trait Hal {
    fn initialize(&mut self) -> crate::Result<()>;
    fn is_ready(&self) -> bool;
    fn shutdown(&mut self);
}
