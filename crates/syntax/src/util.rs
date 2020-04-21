use std::{cell::RefCell, sync::Arc};
pub type P<T> = Arc<T>;
pub struct Mut<T>(Arc<RefCell<T>>);

impl<T> Mut<T> {
    pub fn new(val: T) -> Mut<T> {
        Mut(Arc::new(RefCell::new(val)))
    }
}
