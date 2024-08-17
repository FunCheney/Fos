//! os/src/loader.rs
/// 实现应用的加载

/// 获取链接到内核应用数目
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }

    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// 根据传入的 appid 获取对应的 ELF 可执行文件
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }

    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    assert!(app_id < num_app);

    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
