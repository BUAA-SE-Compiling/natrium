// #[cfg(parse)]
pub mod io;

use crate::opcodes::Op;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// S0 Assembly for use in R0VM
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct S0 {
    pub globals: Vec<GlobalValue>,
    pub functions: Vec<FnDef>,
}

impl Display for S0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for global in &self.globals {
            writeln!(f, "{}", global)?;
        }
        writeln!(f)?;
        for func in &self.functions {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

/// Global variable or constant, described by bytes, addressed by ID
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct GlobalValue {
    pub is_const: bool,
    pub bytes: Vec<u8>,
}

impl Display for GlobalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_const {
            write!(f, "const:")?;
        } else {
            write!(f, "static:")?;
        }
        for byte in &self.bytes {
            write!(f, " {:X}", byte)?;
        }
        if let Ok(s) = String::from_utf8(self.bytes.clone()) {
            write!(f, " (`{}`)", s.escape_default())?;
        }
        writeln!(f)
    }
}

/// Function definition
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub struct FnDef {
    pub name: u32,
    pub ret_slots: u32,
    pub param_slots: u32,
    pub loc_slots: u32,
    pub ins: Vec<Op>,
}

impl Display for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "fn [{}] {} {} -> {} {{",
            self.name, self.loc_slots, self.param_slots, self.ret_slots
        )?;
        for (idx, op) in self.ins.iter().enumerate() {
            writeln!(f, "{:5}: {:?}", idx, op)?;
        }
        writeln!(f, "}}")
    }
}
