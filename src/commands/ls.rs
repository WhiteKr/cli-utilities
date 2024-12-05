use std::{fs, path::Path};

use crate::CliResult;

pub fn execute(path: Option<&str>) -> CliResult<()> {
    let target_path = path.unwrap_or(".");
    let path = Path::new(target_path);

    let mut entries: Vec<_> = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect();

    // Sort entries alphabetically
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        if let Some(name) = entry.file_name().to_str() {
            let file_type = if entry.file_type()?.is_dir() { "/" } else { "" };
            println!("{}{}", name, file_type);
        }
    }

    Ok(())
}
