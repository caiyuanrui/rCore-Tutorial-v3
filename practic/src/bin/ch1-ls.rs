fn main() -> std::io::Result<()> {
    let entries = std::fs::read_dir(".")?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}
