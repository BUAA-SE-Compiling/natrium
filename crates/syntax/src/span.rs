use std::{fmt::Debug, ops::Index};

/// A Span is the information of a piece of source code inside a file.
///
/// `Span`s are only meaningful when indexing the file it is originated from.
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    /// The start index (in bytes or other meaningful item index)
    /// in the file of this span
    pub idx: usize,

    /// The length of the span
    pub len: usize,
}

pub const DUMMY_SPAN: Span = Span {
    // ctx: usize::max_value(),
    idx: 0,
    len: 0,
};

impl Span {
    pub fn start(&self) -> usize {
        self.idx
    }

    pub fn end(&self) -> usize {
        self.idx + self.len
    }

    pub fn new(idx: usize, len: usize) -> Span {
        Span { idx, len }
    }

    pub fn new_idx(lo: usize, hi: usize) -> Span {
        let (lo, hi) = if lo > hi { (hi, lo) } else { (lo, hi) };
        let len = hi - lo;
        Span { idx: lo, len }
    }

    pub const fn eof() -> Span {
        Span {
            idx: usize::max_value(),
            len: 0,
        }
    }
}

impl std::ops::Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let start = std::cmp::min(self.start(), rhs.start());
        let end = std::cmp::max(self.end(), rhs.end());
        Span::new_idx(start, end)
    }
}

impl std::ops::AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.idx, self.idx + self.len)
    }
}

impl Default for Span {
    fn default() -> Self {
        DUMMY_SPAN
    }
}

impl<T> Index<Span> for Vec<T> {
    type Output = [T];
    fn index(&self, index: Span) -> &Self::Output {
        &self[index.idx..(index.idx + index.len)]
    }
}

impl From<logos::Span> for Span {
    fn from(s: logos::Span) -> Self {
        Span::new_idx(s.start, s.end)
    }
}
