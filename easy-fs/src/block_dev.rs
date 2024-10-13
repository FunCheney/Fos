use core::any::Any;

/// trait BlockDevice
/// 在块中读写数据
pub trait BlockDevice: Send + Sync + Any {
    /// 读数据
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    /// 写数据
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
