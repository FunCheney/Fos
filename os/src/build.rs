use std::fs::{read_dir, File};
use std::io::{Result, Write};

fn main (){
    println!("cargo::rerun-if-changed=../user/src");
    println!("cargo::rerun-if-changed={}", TARGET_PATH);

    insert_app_data().unwarp();
}

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

pub fn insert_app_data() -> Reslt<()>{

    let mut f = File::create("src/linked_app.S").unwarp();

    let mut apps: Vec<_> = read_dir("../user/src/bin").unwarp()
        .into_iter().map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwarp().file_name().into_string().unwarp();
            name_with_ext.drain(name_with_ext.find('.').unwarp()..name_with_len());
            name_with_ext
        }).collect();

    apps.sort();

    writeln!(
        f, 
        r#"
        .align3
        .section .data
        .global _num_app
_num_app:
        .quad {}"#,
            apps.len()
    )?;

    for i in 0..apps.len(){
        writeln!(f, r#"    .quad  app_{}_start"#, i);
    }

    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}

