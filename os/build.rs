//! Building applications linker

use std::fs::{read_dir, File};
use std::io::{Result, Write};

fn main() {
    // 指示 Cargo 在哪些文件变化时重新运行脚本。
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

// 应用程序文件所在的路径
static TARGET_PATH: &str = "../user/build/bin/";

/// get app data and build linker 
/// 用于获取应用程序数据并构建链接器。
fn insert_app_data() -> Result<()> {
    // 创建了一个名为 link_app.S 的文件，用于存储链接器的内容
    let mut f = File::create("src/link_app.S").unwrap();
    // 将应用程序名称加入到一个向量中，并按字母顺序进行排序
    let mut apps: Vec<_> = read_dir("../user/build/bin/")
        .unwrap()
        .into_iter()
        // 获取文件名后缀前的部分的操作
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect(); // 收集为vec<String>
    // collect 方法会根据迭代器的类型和目标集合类型，
    // 将迭代器的元素逐个添加到新的集合中
    apps.sort();
    // 向 link_app.S 文件写入链接器的内容，
    // 包括应用程序数量和各应用程序的起始和结束地址
    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len() // 7
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
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
