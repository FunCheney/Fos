use core::any::Any;

/// trait BlockDevice
/// 在块中读写数据
pub trait BlockDevice: Send + Sync + Any {
    /// 读数据
    /// read_block 将编号为 block_id 的数据从磁盘读到内存中的缓冲区
    fn read_block(&self, block_id: usize, buf: &mut [u8]);

    /// 写数据
    /// write_block 将缓冲区中的数据写入到磁盘编号为 block_id 的块中
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
