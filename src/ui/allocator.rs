use std::iter::repeat;

pub struct Allocator {
    // The map of free and used slots in the Vbo. Each byte in this array represents one
    // slot in the Vbo, i.e. the space needed for a single Element. True means used;
    // false means free.
    alloc: Vec<bool>,
    
    // The index to start from the next time we search the map for a free slot.
    alloc_idx: usize
}

impl Allocator {
    pub fn new(buffer_size: usize) -> Allocator {
        Allocator {alloc: repeat(false).take(buffer_size).collect(), alloc_idx: 0}
    }
    
    // Finds the next free slot in the Vbo and marks it as used.
    fn alloc_slot(&mut self) -> usize {
        let mut idx: usize = self.alloc_idx;
        loop {
            if !self.alloc[idx] {
                self.alloc[idx] = true;
                return idx;
            }
            idx += 1;
            if idx == self.alloc_idx {
                // We searched the entire range, wrapping around and returning to the
                // start index, and found no free slot. So we're out of space.
                panic!("all Vbo full");
            }
            if idx == self.alloc.len() { idx = 0; }
        }
    }
    
    fn free_slot(&mut self, idx: usize) {
        self.alloc[idx] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn alloc_slot() {
        let mut all = Allocator::new(3);
        
        // All slots are initially free.
        assert!(!all.alloc[0]);
        assert!(!all.alloc[1]);
        assert!(!all.alloc[2]);
        
        // Alloc slot 0.
        assert_eq!(0, all.alloc_slot());
        assert!( all.alloc[0]);
        assert!(!all.alloc[1]);
        assert!(!all.alloc[2]);
        
        // Alloc slot 1.
        assert_eq!(1, all.alloc_slot());
        assert!( all.alloc[0]);
        assert!( all.alloc[1]);
        assert!(!all.alloc[2]);
        
        // Alloc slot 2.
        assert_eq!(2, all.alloc_slot());
        assert!( all.alloc[0]);
        assert!( all.alloc[1]);
        assert!( all.alloc[2]);
        
        // Free slot 1.
        all.free_slot(1);
        assert!( all.alloc[0]);
        assert!(!all.alloc[1]);
        assert!( all.alloc[2]);
        
        // Free slot 2.
        all.free_slot(2);
        assert!( all.alloc[0]);
        assert!(!all.alloc[1]);
        assert!(!all.alloc[2]);
        
        // Wrap around and alloc slot 1.
        assert_eq!(1, all.alloc_slot());
        assert!( all.alloc[0]);
        assert!( all.alloc[1]);
        assert!(!all.alloc[2]);
    }
}