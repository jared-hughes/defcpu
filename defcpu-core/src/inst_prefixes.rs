use std::fmt;

pub struct DisassemblyPrefix {
    pub address_size: Option<AddressSizeAttribute>,
    pub operand_size: Option<OperandSizeAttribute>,
    pub rex: Option<Rex>,
}
impl fmt::Display for DisassemblyPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut some = false;
        if let Some(s) = &self.address_size {
            s.fmt(f)?;
            some = true;
        }
        if let Some(s) = &self.operand_size {
            if some {
                write!(f, " ")?;
            }
            s.fmt(f)?;
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

/// There are three relevant encodings of instructions
///   A: Just has a ModR/M byte.
///      - Vol 2A: Figure 2-4. "Memory Addressing Without an SIB Byte; REX.X Not Used"
///      - Vol 2A: Figure 2-5. "Register-Register Addressing (No Memory Operand); REX.X Not Used"
///   B: Has a ModR/M byte and a SIB byte.
///      - Vol 2A: Figure 2-6. "Memory Addressing With a SIB Byte"
///   C: Has no ModR/M byte, but the lower 3 bits of the opcode are a reg field.
///      - Vol 2A: Figure 2-7. "Register Operand Coded in Opcode Byte; REX.X & REX.R Not Used"
#[derive(Clone, Copy)]
pub struct Prefix {
    /// true if the Operand-Size Prefix 0x66 is present
    pub operand_size_prefix: bool,
    /// true if the Address-Size Prefix 0x67 is present.
    pub address_size_prefix: bool,
    /// If Some(Rex), a REX prefix is present.
    pub rex: Option<Rex>,
}

#[derive(Clone, Copy)]
pub struct Rex {
    /// If false, operand size is determined by CS.D
    /// If true forces a 64 Bit Operand Size
    pub w: bool,
    /// An extra bit extended to the MSB of the reg field of the ModR/M byte,
    /// when present (encodings A and B), ignored otherwise.
    #[allow(unused)]
    pub r: bool,
    /// An extra bit extended to the MSB of the index field of the SIB byte,
    /// when present (encoding B), ignored otherwise.
    #[allow(unused)]
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

// TODO: test what happens with chains like 0x66 + 0x67 + REX + 0x67 + REX
// Reference says only a single REX can influence, but I'm not sure about
// the other prefixes.
impl Prefix {
    pub fn new() -> Prefix {
        Prefix {
            operand_size_prefix: false,
            address_size_prefix: false,
            rex: None,
        }
    }

    pub fn with_operand_size_prefix(self) -> Prefix {
        let mut p = self;
        p.operand_size_prefix = true;
        p
    }

    pub fn with_address_size_prefix(self) -> Prefix {
        let mut p = self;
        p.address_size_prefix = true;
        p
    }

    pub fn with_rex(self, rex: u8) -> Prefix {
        let mut p = self;
        p.rex = Some(Rex {
            w: rex & 0b1000 != 0,
            r: rex & 0b0100 != 0,
            x: rex & 0b0010 != 0,
            b: rex & 0b0001 != 0,
        });
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
impl AddressSizeAttribute {
    pub fn from_prefix(p: Prefix) -> AddressSizeAttribute {
        if p.address_size_prefix {
            AddressSizeAttribute::Addr32
        } else {
            AddressSizeAttribute::Addr64
        }
    }
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
#[derive(Clone, Copy)]
pub enum OperandSizeAttribute {
    Data64,
    Data32,
    Data16,
}
impl OperandSizeAttribute {
    pub fn from_prefix(p: Prefix) -> OperandSizeAttribute {
        if p.rex.map(|x| x.w).unwrap_or(false) {
            OperandSizeAttribute::Data64
        } else if p.operand_size_prefix {
            OperandSizeAttribute::Data16
        } else {
            OperandSizeAttribute::Data32
        }
    }
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
