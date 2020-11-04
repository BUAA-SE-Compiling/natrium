use bit_set::BitSet;
use r0syntax::span::Span;

use crate::{
    code::BasicBlock,
    err::{CompileError, CompileErrorKind},
};

use super::CompileResult;

/// Cycle Finding state variable
#[derive(Debug)]
pub struct BBArranger<'st> {
    bb: &'st [BasicBlock],
    path: BitSet,
    vis: BitSet,
    in_degree: Vec<usize>,
    arr: Vec<usize>,
}

impl<'st> BBArranger<'st> {
    pub fn new(bb: &'st [BasicBlock]) -> BBArranger<'st> {
        BBArranger {
            bb,
            path: BitSet::new(),
            vis: BitSet::new(),
            in_degree: vec![0; bb.len()],
            arr: vec![],
        }
    }

    pub fn construct_arrangement(&mut self, start: usize) -> CompileResult<()> {
        self.vis(start);
        self.arr(start)
    }

    pub fn vis(&mut self, id: usize) {
        if self.path.contains(id) {
            // cycle does not count
            return;
        }
        self.in_degree[id] += 1;
        if self.vis.contains(id) {
            // visited node
            return;
        }
        self.vis.insert(id);
        self.path.insert(id);
        match self.bb[id].jump {
            crate::code::JumpInst::Jump(bb1) => {
                self.vis(bb1);
            }
            crate::code::JumpInst::JumpIf(bb1, bb2) => {
                self.vis(bb1);
                self.vis(bb2);
            }
            _ => {}
        }
        self.path.remove(id);
    }

    pub fn arr(&mut self, id: usize) -> CompileResult<()> {
        if self.path.contains(id) {
            // cycle does not count
            return Ok(());
        }
        self.in_degree[id] = self.in_degree[id]
            .checked_sub(1)
            .unwrap_or_else(|| panic!("id: {}, in_degrees: {:?}", id, &self.in_degree));
        if self.in_degree[id] != 0 {
            return Ok(());
        }

        self.arr.push(id);
        self.path.insert(id);

        match self.bb[id].jump {
            crate::code::JumpInst::Jump(bb1) => {
                self.arr(bb1)?;
            }
            crate::code::JumpInst::JumpIf(bb1, bb2) => {
                self.arr(bb1)?;
                self.arr(bb2)?;
            }
            crate::code::JumpInst::Return => {}
            crate::code::JumpInst::Unreachable => panic!(
                "Unreachable basic block {} being visited; block map:\n {:#?}",
                id, self.bb
            ),
            crate::code::JumpInst::Undefined => {
                return Err(CompileError(CompileErrorKind::NotAllRoutesReturn, None))
            }
        }
        self.path.remove(id);
        Ok(())
    }

    pub fn arrange(self) -> Vec<usize> {
        self.arr
    }
}
