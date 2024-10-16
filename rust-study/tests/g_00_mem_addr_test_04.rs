#![feature(ptr_metadata)]

//! 参考文档：
//! 1. https://iandouglasscott.com/2018/05/28/exploring-rust-fat-pointers/
//! 2. https://doc.rust-lang.org/nightly/std/ptr/trait.Pointee.html#pointer-metadata
//! 3. https://cheats.rs/#memory-lifetimes
//! 4. https://blog.thoughtram.io/string-vs-str-in-rust/
//! 5. https://rustmagazine.github.io/rust_magazine_2021/chapter_6/ant-rust-data-layout.html

use std::ptr;

pub struct Person {
    name: String,
    age: i32,
}

#[test]
fn test_01() {
    let p = Person {
        name: "Fchen".to_string(),
        age: 18,
    };

    let person = Box::new(p);
    // 获取指向 vtable 的指针和对象的类型信息
    let ptr = Box::into_raw(person) as *const ();
    let metadata = unsafe {
        let vtable_ptr = *(ptr as *const *const ()).offset(0);
        (vtable_ptr, ptr)
    };

    println!("VTable pointer: {:?}", metadata.0);
    println!("Object pointer: {:?}", metadata.1);
}

pub struct MyStruct<'a> {
    value: i32,
    slice: &'a [u8],
}

#[test]
fn test_02() {
    let v = vec![1, 2, 3];
    let my_struct = MyStruct {
        value: 40,
        slice: &v
    };


    // 打印结构体的大小
    println!("Size of MyStruct: {}", std::mem::size_of::<MyStruct>());
    // 打印最后一个字段的大小
    println!("Size of slice: {}", std::mem::size_of_val(my_struct.slice));
    // 获取指向结构体的指针
    let struct_ptr: *const MyStruct = &my_struct;
    // 获取结构体的元数据
    let meta = unsafe { ptr::metadata(struct_ptr) };
    // 打印指针和元数据
    println!("Pointer to MyStruct: {:?}", struct_ptr);
    println!("Metadata for struct pointer: {:?}", meta);

    let v_ptr = v.as_ptr();
    let v_meta = unsafe {ptr::metadata(v_ptr)};

    println!("Pointer to v: {:?}", v_ptr);
    println!("Metadata for v: {:?}", v_meta);



    println!("MetadataTest for v: {:?}", std::ptr::metadata(&v));
}