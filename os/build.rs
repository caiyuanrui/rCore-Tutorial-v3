use std::{
    fs::{File, read_dir},
    io::{Result, Write},
};

fn main() {
    println!("carogo:rerun-if-changed=../user/src/");
    println!("carogo:rerun-if-changed={TARGET_PATH}");
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/build/bin/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S")?;
    let mut apps: Vec<_> = read_dir(TARGET_PATH)?
        .map(|entry| {
            let mut name_with_ext = entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.truncate(name_with_ext.find(".").unwrap());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r"    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}",
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r"    .quad app_{}_start", i)?;
    }
    writeln!(f, r"    .quad app_{}_end", apps.len() - 1)?;

    for (id, app) in apps.iter().enumerate() {
        println!("app_{}: {}", id, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            id, app, TARGET_PATH
        )?;
    }

    Ok(())
}
