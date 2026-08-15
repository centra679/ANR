/// Bounded slab allocator for a single brain section.
/// Implements: SD-07 bounded allocation, free list, compact/defrag.
use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllocId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocPriority {
    Background = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

struct Allocation {
    id: AllocId,
    offset: u64,
    size: u64,
    priority: AllocPriority,
}

pub struct Allocator {
    total_bytes: u64,
    used_bytes: u64,
    allocations: Vec<Allocation>,
    free_list: Vec<(u64, u64)>,
    next_id: u64,
}

impl Allocator {
    pub fn new(total_bytes: u64) -> Self {
        let mut free_list = Vec::new();
        if total_bytes > 0 {
            free_list.push((0, total_bytes));
        }
        Self {
            total_bytes,
            used_bytes: 0,
            allocations: Vec::new(),
            free_list,
            next_id: 1,
        }
    }

    pub fn allocate(&mut self, size: u64, priority: AllocPriority) -> crate::Result<AllocId> {
        if size == 0 {
            return Err(Error::MemoryAllocationFailed(
                "cannot allocate zero bytes".into(),
            ));
        }
        if size > self.total_bytes {
            return Err(Error::MemoryAllocationFailed(format!(
                "requested {} exceeds total capacity {}",
                size, self.total_bytes
            )));
        }
        if self.used_bytes + size > self.total_bytes {
            return Err(Error::MemoryAllocationFailed(format!(
                "insufficient free memory: need {} but only {} available",
                size,
                self.total_bytes - self.used_bytes
            )));
        }

        let best_fit = self
            .free_list
            .iter()
            .enumerate()
            .filter(|(_, &(_, s))| s >= size)
            .min_by_key(|(_, &(_, s))| s)
            .map(|(idx, _)| idx);

        match best_fit {
            Some(idx) => {
                let (offset, block_size) = self.free_list[idx];
                self.free_list.remove(idx);

                let alloc_offset = offset;
                if block_size > size {
                    self.free_list.push((offset + size, block_size - size));
                }

                let id = AllocId(self.next_id);
                self.next_id += 1;
                self.allocations.push(Allocation {
                    id,
                    offset: alloc_offset,
                    size,
                    priority,
                });
                self.used_bytes += size;
                Ok(id)
            }
            None => Err(Error::MemoryAllocationFailed(format!(
                "no contiguous block of {} bytes available",
                size
            ))),
        }
    }

    pub fn free(&mut self, id: AllocId) -> crate::Result<()> {
        let idx = self
            .allocations
            .iter()
            .position(|a| a.id == id)
            .ok_or_else(|| {
                Error::MemoryAllocationFailed(format!("invalid allocation id {:?}", id))
            })?;

        let alloc = self.allocations.remove(idx);
        self.used_bytes -= alloc.size;

        let new_block = (alloc.offset, alloc.size);
        self.free_list.push(new_block);
        self.coalesce_free_list();
        Ok(())
    }

    pub fn used_bytes(&self) -> u64 {
        self.used_bytes
    }

    pub fn free_bytes(&self) -> u64 {
        self.total_bytes - self.used_bytes
    }

    pub fn fragmentation_ratio(&self) -> f64 {
        if self.free_list.is_empty() || self.total_bytes == 0 {
            return 0.0;
        }
        let largest_free = self.free_list.iter().map(|&(_, s)| s).max().unwrap_or(0);
        let total_free = self.free_list.iter().map(|&(_, s)| s).sum::<u64>();
        if total_free == 0 {
            return 0.0;
        }
        1.0 - (largest_free as f64 / total_free as f64)
    }

    pub fn compact(&mut self) {
        self.allocations
            .sort_by_key(|a| (std::cmp::Reverse(a.priority), a.offset));

        let mut offset = 0u64;
        for alloc in &mut self.allocations {
            alloc.offset = offset;
            offset += alloc.size;
        }

        self.free_list.clear();
        let used = self.used_bytes;
        if used < self.total_bytes {
            self.free_list.push((used, self.total_bytes - used));
        }
    }

    pub fn can_allocate(&self, size: u64) -> bool {
        size > 0 && self.used_bytes + size <= self.total_bytes
    }
}

impl Allocator {
    fn coalesce_free_list(&mut self) {
        self.free_list.sort_by_key(|&(offset, _)| offset);
        let mut merged = Vec::new();
        for &(offset, size) in &self.free_list {
            if let Some(last) = merged.last_mut() {
                let (last_offset, last_size): &mut (u64, u64) = last;
                if *last_offset + *last_size == offset {
                    *last_size += size;
                    continue;
                }
            }
            merged.push((offset, size));
        }
        self.free_list = merged;
    }
}
