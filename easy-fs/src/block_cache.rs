use crate::BLOCK_SIZE;
use supper::BlockDevice;


pub struct BlockCache {
    cache: [u8; BLOCK_SIZE],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache{
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SIZE];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }

    fn addr_of_offset(&self, offset: usize) -> usize {
        self.cache[offset] as *const _as usize
    }

    pub fn  get_ref<T>(&self, offset: usize) -> &T where T : Sized {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SIZE);
        let addr = self.add_of_offset(offset);
        unsafe {
            &*(addr as *const T)
        }
    }

    pub fn get_mut<T>(&mut self, offset: usize) -> &T where T: Sized {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SIZE);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe {
            &*(addr as *mut T)
        }
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }

    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {

        f(self.get_ref(offset))
    }

    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_mut(offset))
    }

}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}
