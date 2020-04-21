use std::ops::Index;

/// A Span is the information of a piece of source code inside a file.
///
/// `Span`s are only meaningful when indexing the file it is originated from.
pub struct Span {
    /// The context this span is indexing from, in most cases some
    /// representation of a file. Should be consistent throughout the whole
    /// parsing process.
    ///
    /// When this value is `usize::max_value()`, the span is dummy.
    ctx: usize,

    /// The start index (in bytes or other meaningful item index)
    /// in the file of this span
    idx: usize,

    /// The length of the span
    len: usize,
}

pub const DUMMY_SPAN: Span = Span {
    ctx: usize::max_value(),
    idx: 0,
    len: 0,
};

impl Span {
    pub fn new(ctx: usize, idx: usize, len: usize) -> Span {
        Span { ctx, idx, len }
    }

    pub fn new_idx(ctx: usize, lo: usize, hi: usize) -> Span {
        let (lo, hi) = if lo > hi { (hi, lo) } else { (lo, hi) };
        let len = hi - lo;
        Span { ctx, idx: lo, len }
    }

    pub fn is_dummy(&self) -> bool {
        self.ctx == usize::max_value()
    }
}

impl<T> Index<Span> for Vec<T> {
    type Output = [T];
    fn index(&self, index: Span) -> &Self::Output {
        &self[index.idx..(index.idx + index.len)]
    }
}
