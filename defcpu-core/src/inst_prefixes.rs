use std::fmt;

use crate::inst::Group1Prefix;

#[derive(Clone, Copy)]
enum VarDisPrefix {
    AddressSize(AddressSizeAttribute),
    OperandSize(OperandSizeAttribute),
    Group1(Group1Prefix),
}

#[derive(Clone)]
pub struct DisassemblyPrefix {
    prefixes: Vec<VarDisPrefix>,
    rex: Option<Rex>,
}
impl fmt::Display for DisassemblyPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut some = false;
        for vdp in &self.prefixes {
            if some {
                write!(f, " ")?;
            }
            match vdp {
                VarDisPrefix::AddressSize(s) => s.fmt(f)?,
                VarDisPrefix::OperandSize(s) => s.fmt(f)?,
                VarDisPrefix::Group1(group1_prefix) => group1_prefix.fmt(f)?,
            };
            some = true;
        }
        if let Some(rex) = &self.rex {
            if some {
                write!(f, " ")?;
            }
            rex.fmt(f)?;
        }
        Ok(())
    }
}
impl DisassemblyPrefix {
    pub fn new() -> DisassemblyPrefix {
        DisassemblyPrefix {
            prefixes: vec![],
            rex: None,
        }
    }

    fn push_prefix(&mut self, prefix: VarDisPrefix) {
        self.prefixes.push(prefix);
    }

    pub fn set_rex(&mut self, rex: Option<Rex>) {
        self.rex = rex;
    }

    /// Delete the last AddressSize(_) prefix
    pub fn remove_last_address_size_prefix(&mut self) {
        for i in (0..self.prefixes.len()).rev() {
            if let VarDisPrefix::AddressSize(_) = self.prefixes[i] {
                let mut new_vec = vec![];
                // splice
                for j in 0..self.prefixes.len() {
                    if j != i {
                        new_vec.push(self.prefixes[j]);
                    }
                }
                self.prefixes = new_vec;
                return;
            }
        }
    }

    /// Delete the last OperandSize(_) prefix
    pub fn remove_last_operand_size_prefix(&mut self) {
        for i in (0..self.prefixes.len()).rev() {
            if let VarDisPrefix::OperandSize(_) = self.prefixes[i] {
                let mut new_vec = vec![];
                // splice
                for j in 0..self.prefixes.len() {
                    if j != i {
                        new_vec.push(self.prefixes[j]);
                    }
                }
                self.prefixes = new_vec;
                return;
            }
        }
    }

    /// Delete the Rex prefix
    pub fn remove_rex_prefix(&mut self) {
        self.rex = None;
    }
}

/// There are three relevant encodings of instructions to Rex
///   A: Just has a ModR/M byte.
///      - Vol 2A: Figure 2-4. "Memory Addressing Without an SIB Byte; REX.X Not Used"
///      - Vol 2A: Figure 2-5. "Register-Register Addressing (No Memory Operand); REX.X Not Used"
///   B: Has a ModR/M byte and a SIB byte.
///      - Vol 2A: Figure 2-6. "Memory Addressing With a SIB Byte"
///   C: Has no ModR/M byte, but the lower 3 bits of the opcode are a reg field.
///      - Vol 2A: Figure 2-7. "Register Operand Coded in Opcode Byte; REX.X & REX.R Not Used"
#[derive(Clone, Copy)]
pub struct Rex {
    /// If false, operand size is determined by CS.D
    /// If true forces a 64 Bit Operand Size
    pub w: bool,
    /// An extra bit extended to the MSB of the reg field of the ModR/M byte,
    /// when present (encodings A and B), ignored otherwise.
    pub r: bool,
    /// An extra bit extended to the MSB of the index field of the SIB byte,
    /// when present (encoding B), ignored otherwise.
    pub x: bool,
    /// An extra bit extended to the MSB of the:
    ///   - Encoding A: R/M field of ModR/M byte.
    ///   - Encoding B: base field of SIB byte.
    ///   - Encoding C: reg field of the opcode.
    pub b: bool,
}
impl Rex {}
impl fmt::Display for Rex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rex")?;
        if self.w || self.r || self.x || self.b {
            write!(f, ".")?;
        };
        if self.w {
            write!(f, "W")?;
        }
        if self.r {
            write!(f, "R")?;
        }
        if self.x {
            write!(f, "X")?;
        }
        if self.b {
            write!(f, "B")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Prefix {
    /// None if there is no such prefix, or if it was mandatory for an instruction like popcnt.
    /// Otherwise, the most recent Group 1 prefix (rep, repz, repnz).
    pub group1_prefix: Option<Group1Prefix>,
    /// Data16 if the Operand-Size Prefix 0x66 is present, otherwise Data32.
    /// Unaffected by any REX bit.
    /// Operand_size is the only Group 3 Prefix.
    pub operand_size: OperandSizeAttribute,
    /// Addr32 if the Address-Size Prefix 0x67 is present, otherwise Addr64
    /// Note this doesn't factor in the REX.W bit, since the behavior of that depends per-instruction.
    /// Operand_size is the only Group 4 Prefix.
    pub address_size: AddressSizeAttribute,
    /// If Some(Rex), a REX prefix is present.
    pub rex: Option<Rex>,
    /// For disassembly, provides the order of prefixes
    pub dis_prefix: DisassemblyPrefix,
}

// TODO: test what happens with chains like 0x66 + 0x67 + REX + 0x67 + REX
// Reference says only a single REX can influence, but I'm not sure about
// the other prefixes.
impl Prefix {
    pub fn new() -> Prefix {
        Prefix {
            group1_prefix: None,
            operand_size: OperandSizeAttribute::Data32,
            address_size: AddressSizeAttribute::Addr64,
            rex: None,
            dis_prefix: DisassemblyPrefix::new(),
        }
    }

    pub fn with_operand_size_prefix(&self) -> Prefix {
        let mut p = self.clone();
        p.operand_size = OperandSizeAttribute::Data16;
        p.dis_prefix
            .push_prefix(VarDisPrefix::OperandSize(p.operand_size));
        p
    }

    pub fn with_address_size_prefix(&self) -> Prefix {
        let mut p = self.clone();
        p.address_size = AddressSizeAttribute::Addr32;
        p.dis_prefix
            .push_prefix(VarDisPrefix::AddressSize(p.address_size));
        p
    }

    pub fn with_group1_prefix(&self, group1_prefix: Group1Prefix) -> Prefix {
        let mut p = self.clone();
        p.group1_prefix = Some(group1_prefix);
        p.dis_prefix
            .push_prefix(VarDisPrefix::Group1(group1_prefix));
        p
    }

    pub fn with_rex(&self, rex: u8) -> Prefix {
        let mut p = self.clone();
        p.rex = Some(Rex {
            w: rex & 0b1000 != 0,
            r: rex & 0b0100 != 0,
            x: rex & 0b0010 != 0,
            b: rex & 0b0001 != 0,
        });
        p.dis_prefix.set_rex(p.rex);
        p
    }
}

/// Vol 1. 3.6.1 "Operand Size and Address Size in 64-Bit Mode"
/// Address size is 64 by default, but 32 if the Address-Size Prefix 0x67 is present.
#[derive(Clone, Copy)]
pub enum AddressSizeAttribute {
    Addr64,
    Addr32,
}
impl fmt::Display for AddressSizeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressSizeAttribute::Addr64 => write!(f, "addr64"),
            AddressSizeAttribute::Addr32 => write!(f, "addr32"),
        }
    }
}

/// Vol 1. 3.6.1 "Operand Size and Address Size in 64-Bit Mode"
/// Operand size is 32 by default, but 64 if the REX.W prefix is present,
/// else 16 if the Operand-Size Prefix 0x66 is present. The REX.W takes precedence.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OperandSizeAttribute {
    Data64,
    Data32,
    Data16,
}
impl fmt::Display for OperandSizeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandSizeAttribute::Data64 => write!(f, "data64"),
            OperandSizeAttribute::Data32 => write!(f, "data32"),
            OperandSizeAttribute::Data16 => write!(f, "data16"),
        }
    }
}
