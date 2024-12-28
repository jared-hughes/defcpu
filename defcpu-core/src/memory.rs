use std::{collections::HashMap, fmt};

use crate::parse_elf::{LoadSegment, PermissionFlags};

const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = PAGE_SIZE - 1;

struct Page {
    data: [u8; PAGE_SIZE],
    flags: PermissionFlags,
}

pub struct Memory {
    map: HashMap<usize, Page>,
}

/** Return 0 if all page entries are 0, otherwise the index after the last nonzero. */
fn after_last_nonzero(page: &Page) -> usize {
    for i in (0..PAGE_SIZE).rev() {
        if page.data[i] != 0 {
            return i + 1;
        }
    }
    0
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (page_addr, page) in self.map.iter() {
            if first {
                first = false
            } else {
                writeln!(f)?;
            }
            writeln!(f, "Page at 0x{:016X} ({}):", page_addr, page.flags)?;
            let len = after_last_nonzero(page);
            let nlines = len.div_ceil(16);
            for i in 0..nlines {
                for j in 0..16 {
                    let ind = i * 16 + j;
                    if ind < len {
                        write!(f, "{:02X}", page.data[ind])?;
                    } else {
                        write!(f, "  ")?;
                    }
                    if j % 2 == 1 {
                        write!(f, " ")?;
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Memory {
    pub fn from_segments(segments: &[LoadSegment]) -> Memory {
        let mut mem = Memory {
            map: HashMap::new(),
        };
        for segment in segments {
            let data = segment.segment_data;
            let len = data.len();
            let npages = len.div_ceil(PAGE_SIZE);
            // TODO: I wonder if the lengths specified in the phdr could differ from the data.len().
            // TODO: do we need a page for a 0-len segment?
            for i in 0..npages {
                let offset = PAGE_SIZE * i;
                let addr = (segment.p_vaddr as usize) + offset;
                let page_addr = addr & !PAGE_MASK;
                if mem.map.contains_key(&page_addr) {
                    panic!("Duplicate page 0x{:016X}", page_addr);
                }
                let mut page_data = [0_u8; PAGE_SIZE];
                let suffix_slice = &data[offset..];
                let copy_len = suffix_slice.len().min(PAGE_SIZE);
                page_data[..copy_len].copy_from_slice(&suffix_slice[..copy_len]);
                let page = Page {
                    data: page_data,
                    flags: segment.flags,
                };
                mem.map.insert(page_addr, page);
            }
        }
        mem
    }

    pub fn read_u8(&self, i: u64) -> u8 {
        let page = self.get_page(i);
        if !page.flags.readable {
            panic!("Page not writeable at address 0x{:016X}.", i);
        }
        page.data[(i as usize) & PAGE_MASK]
    }

    pub fn read_u16(&self, i: u64) -> u16 {
        let d0 = self.read_u8(i + 0) as u16;
        let d1 = self.read_u8(i + 1) as u16;
        (d1 << 8) | d0
    }

    pub fn read_u32(&self, i: u64) -> u32 {
        let d0 = self.read_u8(i + 0) as u32;
        let d1 = self.read_u8(i + 1) as u32;
        let d2 = self.read_u8(i + 2) as u32;
        let d3 = self.read_u8(i + 3) as u32;
        (d3 << 24) | (d2 << 16) | (d1 << 8) | d0
    }

    pub fn read_u64(&self, i: u64) -> u64 {
        let d0 = self.read_u8(i + 0) as u64;
        let d1 = self.read_u8(i + 1) as u64;
        let d2 = self.read_u8(i + 2) as u64;
        let d3 = self.read_u8(i + 3) as u64;
        let d4 = self.read_u8(i + 4) as u64;
        let d5 = self.read_u8(i + 5) as u64;
        let d6 = self.read_u8(i + 6) as u64;
        let d7 = self.read_u8(i + 7) as u64;
        (d7 << 56) | (d6 << 48) | (d5 << 40) | (d4 << 32) | (d3 << 24) | (d2 << 16) | (d1 << 8) | d0
    }

    pub fn write_u8(&mut self, i: u64, val: u8) {
        let page = self.get_page_mut(i);
        if !page.flags.writeable {
            panic!("Page not writeable at address 0x{:016X}.", i);
        }
        page.data[(i as usize) & PAGE_MASK] = val;
    }

    pub fn write_u16(&mut self, i: u64, val: u16) {
        self.write_u8(i + 0, (val & 0xFF).try_into().unwrap());
        self.write_u8(i + 1, (val >> 8).try_into().unwrap());
    }

    pub fn write_u32(&mut self, i: u64, val: u32) {
        self.write_u8(i + 0, (val & 0xFF).try_into().unwrap());
        self.write_u8(i + 1, (val >> 8 & 0xFF).try_into().unwrap());
        self.write_u8(i + 2, (val >> 16 & 0xFF).try_into().unwrap());
        self.write_u8(i + 3, (val >> 24 & 0xFF).try_into().unwrap());
    }

    pub fn write_u64(&mut self, i: u64, val: u64) {
        self.write_u8(i + 0, (val & 0xFF).try_into().unwrap());
        self.write_u8(i + 1, (val >> 8 & 0xFF).try_into().unwrap());
        self.write_u8(i + 2, (val >> 16 & 0xFF).try_into().unwrap());
        self.write_u8(i + 3, (val >> 24 & 0xFF).try_into().unwrap());
        self.write_u8(i + 4, (val >> 32 & 0xFF).try_into().unwrap());
        self.write_u8(i + 5, (val >> 40 & 0xFF).try_into().unwrap());
        self.write_u8(i + 6, (val >> 48 & 0xFF).try_into().unwrap());
        self.write_u8(i + 7, (val >> 56 & 0xFF).try_into().unwrap());
    }

    fn get_page(&self, addr: u64) -> &Page {
        self.map
            .get(&((addr as usize) & !PAGE_MASK))
            .unwrap_or_else(|| page_fault(addr))
    }

    fn get_page_mut(&mut self, addr: u64) -> &mut Page {
        self.map
            .get_mut(&((addr as usize) & !PAGE_MASK))
            .unwrap_or_else(|| page_fault(addr))
    }
}

fn page_fault(addr: u64) -> ! {
    panic!(
        "Page fault: not yet initialized. Reading address 0x{:016X}.",
        addr
    )
}
