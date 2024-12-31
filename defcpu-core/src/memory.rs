use std::collections::HashMap;

use crate::parse_elf::{LoadSegment, PermissionFlags};

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

    fn write_u8(&mut self, addr: u64, val: u8) {
        if !self.flags.writeable {
            panic!("Segment not writeable at address {:#016X}.", addr);
        }
        self.write_u8_unchecked(addr, val);
    }

    fn write_u8_unchecked(&mut self, addr: u64, val: u8) {
        let page_data = self
            .page_table
            .entry(page_addr(addr))
            .or_insert_with(|| Box::new([0_u8; PAGE_SIZE]));
        page_data[page_index(addr)] = val;
    }

    fn read_u8(&self, addr: u64) -> u8 {
        if !self.flags.readable {
            panic!("Segment not readable at address {:#016X}.", addr);
        }
        self.read_u8_unchecked(addr)
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
    segments: Vec<MemSegment>,
}
impl Memory {
    pub fn from_segments(segments: &[LoadSegment]) -> Memory {
        let mut mem = Memory::new();
        mem.insert_segments(segments);
        mem
    }

    fn new() -> Memory {
        Memory {
            segments: Vec::new(),
        }
    }

    fn insert_segments(&mut self, segments: &[LoadSegment]) {
        for segment in segments {
            if page_addr(segment.p_vaddr) != segment.p_vaddr {
                panic!("Virtual address is not page-aligned. Haven't thought that out yet.")
            }
            let mut new_seg = MemSegment {
                start: segment.p_vaddr,
                end: segment.p_vaddr + segment.memsz,
                flags: segment.flags,
                page_table: HashMap::new(),
            };
            for other in &self.segments {
                if new_seg.overlaps(other) {
                    panic!(
                        "New segment at {}..{} overlaps the segment at {}..{}",
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

    pub fn write_u8(&mut self, addr: u64, val: u8) {
        let segment = self.segments.iter_mut().find(|seg| seg.contains_addr(addr));
        let Some(segment) = segment else {
            panic!(
                "Segmentation fault: address {:#016X} outside every segment.",
                addr
            );
        };
        segment.write_u8(addr, val);
    }

    pub fn read_u8(&self, addr: u64) -> u8 {
        let segment = self.segments.iter().find(|seg| seg.contains_addr(addr));
        let Some(segment) = segment else {
            panic!(
                "Segmentation fault: address {:#016X} outside every segment.",
                addr
            );
        };
        segment.read_u8(addr)
    }

    pub fn read_u16(&self, i: u64) -> u16 {
        let d0 = self.read_u8(i) as u16;
        let d1 = self.read_u8(i + 1) as u16;
        (d1 << 8) | d0
    }

    pub fn read_u32(&self, i: u64) -> u32 {
        let d0 = self.read_u8(i) as u32;
        let d1 = self.read_u8(i + 1) as u32;
        let d2 = self.read_u8(i + 2) as u32;
        let d3 = self.read_u8(i + 3) as u32;
        (d3 << 24) | (d2 << 16) | (d1 << 8) | d0
    }

    pub fn read_u64(&self, i: u64) -> u64 {
        let d0 = self.read_u8(i) as u64;
        let d1 = self.read_u8(i + 1) as u64;
        let d2 = self.read_u8(i + 2) as u64;
        let d3 = self.read_u8(i + 3) as u64;
        let d4 = self.read_u8(i + 4) as u64;
        let d5 = self.read_u8(i + 5) as u64;
        let d6 = self.read_u8(i + 6) as u64;
        let d7 = self.read_u8(i + 7) as u64;
        (d7 << 56) | (d6 << 48) | (d5 << 40) | (d4 << 32) | (d3 << 24) | (d2 << 16) | (d1 << 8) | d0
    }

    pub fn write_u16(&mut self, i: u64, val: u16) {
        self.write_u8(i, (val & 0xFF).try_into().unwrap());
        self.write_u8(i + 1, (val >> 8).try_into().unwrap());
    }

    pub fn write_u32(&mut self, i: u64, val: u32) {
        self.write_u8(i, (val & 0xFF).try_into().unwrap());
        self.write_u8(i + 1, (val >> 8 & 0xFF).try_into().unwrap());
        self.write_u8(i + 2, (val >> 16 & 0xFF).try_into().unwrap());
        self.write_u8(i + 3, (val >> 24 & 0xFF).try_into().unwrap());
    }

    pub fn write_u64(&mut self, i: u64, val: u64) {
        self.write_u8(i, (val & 0xFF).try_into().unwrap());
        self.write_u8(i + 1, (val >> 8 & 0xFF).try_into().unwrap());
        self.write_u8(i + 2, (val >> 16 & 0xFF).try_into().unwrap());
        self.write_u8(i + 3, (val >> 24 & 0xFF).try_into().unwrap());
        self.write_u8(i + 4, (val >> 32 & 0xFF).try_into().unwrap());
        self.write_u8(i + 5, (val >> 40 & 0xFF).try_into().unwrap());
        self.write_u8(i + 6, (val >> 48 & 0xFF).try_into().unwrap());
        self.write_u8(i + 7, (val >> 56 & 0xFF).try_into().unwrap());
    }
}
