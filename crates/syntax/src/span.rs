use std::ops::Index;

/// A Span is the information of a piece of source code inside a file.
///
/// `Span`s are only meaningful when indexing the file it is originated from.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    /// The start index (in bytes or other meaningful item index)
    /// in the file of this span
    idx: usize,

    /// The length of the span
    len: usize,
}

pub const DUMMY_SPAN: Span = Span {
    // ctx: usize::max_value(),
    idx: 0,
    len: 0,
};

impl Span {
    pub fn new(idx: usize, len: usize) -> Span {
        Span { idx, len }
    }

    pub fn new_idx(lo: usize, hi: usize) -> Span {
        let (lo, hi) = if lo > hi { (hi, lo) } else { (lo, hi) };
        let len = hi - lo;
        Span { idx: lo, len }
    }
}

impl std::ops::Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let len = rhs.idx - self.idx + rhs.len;
        Span::new(self.idx, len)
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
