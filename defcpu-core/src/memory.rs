use std::{cmp::min, collections::HashMap};

use crate::{
    errors::{RResult, Rerr},
    parse_elf::{LoadSegment, PermissionFlags},
};

const PAGE_SIZE: usize = 4096;
fn page_index(addr: u64) -> usize {
    (addr as usize) & (PAGE_SIZE - 1)
}
fn page_addr(addr: u64) -> u64 {
    addr & !((PAGE_SIZE - 1) as u64)
}

struct MemSegment {
    /// Starting virtual address. Inclusive. Multiple of PAGE_SIZE.
    start: u64,
    /// Ending virtual address. Exclusive. Multiple of PAGE_SIZE.
    end: u64,
    flags: PermissionFlags,
    page_table: HashMap<u64, Box<[u8; PAGE_SIZE]>>,
}
impl MemSegment {
    fn contains_addr(&self, addr: u64) -> bool {
        self.start <= addr && addr < self.end
    }

    fn overlaps(&self, other: &MemSegment) -> bool {
        self.contains_addr(other.start) || other.contains_addr(self.start)
    }

    /// Returns the number of bytes written
    fn write_bytes(&mut self, addr: u64, bytes: &[u8]) -> RResult<u64> {
        if !self.flags.writeable {
            Err(Rerr::SegmentNotWriteable(addr))?
        }
        Ok(self.write_bytes_unchecked(addr, bytes))
    }

    fn write_u8(&mut self, addr: u64, val: u8) -> RResult<()> {
        if !self.flags.writeable {
            Err(Rerr::SegmentNotWriteable(addr))?
        }
        self.write_u8_unchecked(addr, val);
        Ok(())
    }

    /// Returns the number of bytes written
    fn write_bytes_unchecked(&mut self, addr: u64, bytes: &[u8]) -> u64 {
        let page_data = self
            .page_table
            .entry(page_addr(addr))
            .or_insert_with(|| Box::new([0_u8; PAGE_SIZE]));
        let ind = page_index(addr);
        let len = min(PAGE_SIZE - ind, bytes.len());
        for i in 0..len {
            page_data[ind + i] = bytes[i];
        }
        len as u64
    }

    fn write_u8_unchecked(&mut self, addr: u64, val: u8) {
        let page_data = self
            .page_table
            .entry(page_addr(addr))
            .or_insert_with(|| Box::new([0_u8; PAGE_SIZE]));
        page_data[page_index(addr)] = val;
    }

    fn read_u8(&self, addr: u64) -> RResult<u8> {
        if !self.flags.readable {
            Err(Rerr::SegmentNotReadable(addr))?
        }
        Ok(self.read_u8_unchecked(addr))
    }

    fn read_u8_unchecked(&self, addr: u64) -> u8 {
        let page_data = self.page_table.get(&page_addr(addr));
        match page_data {
            Some(page_data) => page_data[page_index(addr)],
            // Pages are 0-filled by default
            // TODO: verify that. Do we have 'random' memory instead?
            None => 0,
        }
    }
}

pub struct Memory {
    // TODO-cleanup: rename segment to VMA (virtual memory area)
    segments: Vec<MemSegment>,
}
impl Memory {
    pub(crate) fn new() -> Memory {
        Memory {
            segments: Vec::new(),
        }
    }

    /// Published for 'kernel codee' only, in init_mem.rs.
    pub(crate) fn insert_segments(&mut self, segments: &[LoadSegment]) {
        for segment in segments {
            if page_addr(segment.p_vaddr) != segment.p_vaddr {
                panic!("Virtual address is not page-aligned. Haven't thought that out yet.")
            }
            let memsz = segment.memsz.next_multiple_of(PAGE_SIZE as u64);
            let mut new_seg = MemSegment {
                start: segment.p_vaddr,
                end: segment.p_vaddr + memsz,
                flags: segment.flags,
                page_table: HashMap::new(),
            };
            for other in &self.segments {
                if new_seg.overlaps(other) {
                    panic!(
                        "New segment at {:#x}..{:#x} overlaps the segment at {:#x}..{:#x}",
                        new_seg.start, new_seg.end, other.start, other.end
                    );
                }
            }
            for (i, byte) in segment.segment_data.iter().enumerate() {
                // Unchecked to allow writing even if the segment is not writeable.
                new_seg.write_u8_unchecked(segment.p_vaddr + (i as u64), *byte);
            }
            self.segments.push(new_seg);
        }
    }

    /// Copy `bytes` into the area starting at `to`.
    pub fn write_bytes(&mut self, mut addr: u64, mut bytes: &[u8]) -> RResult<()> {
        let mut segment: Option<&mut MemSegment> = None;
        while !bytes.is_empty() {
            if let Some(ref seg) = segment {
                if !seg.contains_addr(addr) {
                    segment = None;
                }
            }
            if segment.is_none() {
                segment = self.segments.iter_mut().find(|seg| seg.contains_addr(addr));
            }
            let Some(ref mut seg) = segment else {
                return Err(Rerr::WriteOutsideSegment(addr));
            };
            let bytes_written = seg.write_bytes(addr, bytes)?;
            bytes = &bytes[bytes_written as usize..];
            addr += bytes_written;
        }
        Ok(())
    }

    pub fn write_u8(&mut self, addr: u64, val: u8) -> Result<(), Rerr> {
        let segment = self.segments.iter_mut().find(|seg| seg.contains_addr(addr));
        let Some(segment) = segment else {
            return Err(Rerr::WriteOutsideSegment(addr));
        };
        segment.write_u8(addr, val)
    }

    pub fn read_u8(&self, addr: u64) -> RResult<u8> {
        let segment = self.segments.iter().find(|seg| seg.contains_addr(addr));
        let Some(segment) = segment else {
            return Err(Rerr::ReadOutsideSegment(addr));
        };
        segment.read_u8(addr)
    }

    pub fn read_u16(&self, i: u64) -> RResult<u16> {
        let d0 = self.read_u8(i)? as u16;
        let d1 = self.read_u8(i + 1)? as u16;
        Ok((d1 << 8) | d0)
    }

    pub fn read_u32(&self, i: u64) -> RResult<u32> {
        let d0 = self.read_u8(i)? as u32;
        let d1 = self.read_u8(i + 1)? as u32;
        let d2 = self.read_u8(i + 2)? as u32;
        let d3 = self.read_u8(i + 3)? as u32;
        Ok((d3 << 24) | (d2 << 16) | (d1 << 8) | d0)
    }

    pub fn read_u64(&self, i: u64) -> RResult<u64> {
        let d0 = self.read_u8(i)? as u64;
        let d1 = self.read_u8(i + 1)? as u64;
        let d2 = self.read_u8(i + 2)? as u64;
        let d3 = self.read_u8(i + 3)? as u64;
        let d4 = self.read_u8(i + 4)? as u64;
        let d5 = self.read_u8(i + 5)? as u64;
        let d6 = self.read_u8(i + 6)? as u64;
        let d7 = self.read_u8(i + 7)? as u64;
        Ok((d7 << 56)
            | (d6 << 48)
            | (d5 << 40)
            | (d4 << 32)
            | (d3 << 24)
            | (d2 << 16)
            | (d1 << 8)
            | d0)
    }

    pub fn write_u16(&mut self, i: u64, val: u16) -> RResult<()> {
        self.write_u8(i, val as u8)?;
        self.write_u8(i + 1, (val >> 8) as u8)?;
        Ok(())
    }

    pub fn write_u32(&mut self, i: u64, val: u32) -> RResult<()> {
        self.write_u8(i, val as u8)?;
        self.write_u8(i + 1, (val >> 8) as u8)?;
        self.write_u8(i + 2, (val >> 16) as u8)?;
        self.write_u8(i + 3, (val >> 24) as u8)?;
        Ok(())
    }

    pub fn write_u64(&mut self, i: u64, val: u64) -> RResult<()> {
        self.write_u8(i, val as u8)?;
        self.write_u8(i + 1, (val >> 8) as u8)?;
        self.write_u8(i + 2, (val >> 16) as u8)?;
        self.write_u8(i + 3, (val >> 24) as u8)?;
        self.write_u8(i + 4, (val >> 32) as u8)?;
        self.write_u8(i + 5, (val >> 40) as u8)?;
        self.write_u8(i + 6, (val >> 48) as u8)?;
        self.write_u8(i + 7, (val >> 56) as u8)?;
        Ok(())
    }
}
