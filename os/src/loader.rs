//! os/src/loader.rs
use alloc::vec::Vec;
/// 实现应用的加载
use lazy_static::*;

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

lazy_static! {
    static ref APP_NAMES: Vec<&'static str> = {
        // 获取 app 数量
        let num_app = get_num_app();
        extern "C" {
            fn _app_names();
        }

        let mut start = _app_names as usize as *const u8;
        let mut v = Vec::new();
        unsafe {
            for _ in 0..num_app {
                let mut end = start;
                // 读到 ‘\0’ 表示结束本次
                while end.read_volatile() != '\0' as u8 {
                    end = end.add(1);
                }

                let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
                let str = core::str::from_utf8(slice).unwrap();
                v.push(str);
                start = end.add(1);
            }
        }

        v
    };
}

pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    let app_num = get_num_app();
    (0..app_num)
        .find(|&i| APP_NAMES[i] == name)
        .map(|i| get_app_data(i))
}

pub fn list_apps() {
    println!("=====APPS=====");
    // 所有 appname 都保存在 APP_NAMES 中
    for app in APP_NAMES.iter() {
        println!("{}", app);
    }

    println!("^^^###^^^/");
}
