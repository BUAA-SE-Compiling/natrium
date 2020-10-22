use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

pub type P<T> = Rc<T>;

pub struct Mut<T>(Rc<RefCell<T>>);

pub struct MutWeak<T>(std::rc::Weak<RefCell<T>>);

impl<T> Mut<T> {
    pub fn new(val: T) -> Mut<T> {
        Mut(Rc::new(RefCell::new(val)))
    }

    pub fn weak(&self) -> MutWeak<T> {
        MutWeak(Rc::downgrade(&self.0))
    }
}

impl<T> Deref for Mut<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for MutWeak<T> {
    type Target = std::rc::Weak<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Mut<T> {
    fn clone(&self) -> Self {
        Mut(self.0.clone())
    }
}

impl<T> Debug for Mut<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl<T> std::fmt::Display for Mut<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&*self.0.borrow(), f)
    }
}

impl<T> Debug for MutWeak<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
