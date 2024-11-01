//! BlockCache 代表一个被管理起来的缓冲区

use super::{BlockDevice, BLOCK_SZ};
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use spin::Mutex;
/// Cached block inside memory
pub struct BlockCache {
    /// cached block data
    /// 512 字节的数组，表示微云缓冲区中的数组
    cache: [u8; BLOCK_SZ],
    /// underlying block id
    /// 记录块缓存来自磁盘中的块编号
    block_id: usize,
    /// underlying block device
    /// 对底层设备块的引用，可通过他完成快读写
    block_device: Arc<dyn BlockDevice>,
    /// whether the block is dirty
    /// 记录这个块从磁盘载入内存后，有没有被修改过
    modified: bool,
}

impl BlockCache {
    /// Load a new BlockCache from disk.
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SZ];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }
    /// 得到一个 BlockCache 内部的缓冲区中指定偏移量 offset 的字节地址
    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }
    /// 它可以获取缓冲区中的位于偏移量 offset 的一个类型为 T 的磁盘上数据结构的不可变引用
    pub fn get_ref<T>(&self, offset: usize) -> &T
        where
        // 该泛型方法的 Trait Bound 限制类型 T 必须是一个编译时已知大小的类型
            T: Sized,
    {
        // 在编译时获取类型 T 的大小
        let type_size = core::mem::size_of::<T>();
        // 确认该数据结构被整个包含在磁盘块及其缓冲区之内
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    /// get_mut 会获取磁盘上数据结构的可变引用，由此可以对数据结构进行修改
    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
        where
            T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        // 修改标记状态，表示该缓冲区已经被修改
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    /// 在 BlockCache 缓冲区偏移量为 offset 的位置获取一个类型为 T 的磁盘上数据结构的不可变引用
    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    /// 在 BlockCache 缓冲区偏移量为 offset 的位置获取一个类型为 T 的磁盘上数据结构的可变引用
    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}
/// Use a block cache of 16 blocks
const BLOCK_CACHE_SIZE: usize = 16;

/// 块缓存管理器
pub struct BlockCacheManager {
    // 对 BlockCache 进行管理
    queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn get_block_cache(
        &mut self,
        block_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>> {
        if let Some(pair) = self.queue.iter().
            find(|pair| pair.0 == block_id) {
            Arc::clone(&pair.1)
        } else {
            // 判断管理器保存的块缓存数量是否已经达到上限
            if self.queue.len() == BLOCK_CACHE_SIZE {
                // from front to tail
                // 每加入一个块缓存时要从队尾加入；要替换时则从队头弹出。
                // 但此时队头对应的块缓存可能仍在使用：判断的标志是其强引用计数 ≥ 2
                // 即除了块缓存管理器保留的一份副本之外，在外面还有若干份副本正在使用。因此，我们的做法是从队
                // 头遍历到队尾找到第一个强引用计数恰好为 1 的块缓存并将其替换出去
                if let Some((idx, _)) = self
                    .queue
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    // 需要执行缓存替换算法，丢掉某个块缓存并空出一个空位
                    self.queue.drain(idx..=idx);
                } else {
                    panic!("Run out of BlockCache!");
                }
            }
            // load block into mem and push back
            let block_cache = Arc::new(Mutex::new(BlockCache::new(
                block_id,
                Arc::clone(&block_device),
            )));
            // 添加到队尾
            self.queue.push_back((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }
}

lazy_static! {
    /// The global block cache manager
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> =
        Mutex::new(BlockCacheManager::new());
}
/// Get the block cache corresponding to the given block id and block device
/// 提供给其他模块使用，获取块缓存
/// 返回 Arc<Mutex<BlockCahe>> , 调用方通过 .lock() 获取互斥锁 Mutex，才能对里面的 BlockCache 进行操作
/// 通过 read/ modify 访问缓冲区里面的数据
pub fn get_block_cache(
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHE_MANAGER
        .lock()
        .get_block_cache(block_id, block_device)
}
/// Sync all block cache to block device
pub fn block_cache_sync_all() {
    let manager = BLOCK_CACHE_MANAGER.lock();
    for (_, cache) in manager.queue.iter() {
        cache.lock().sync();
    }
}