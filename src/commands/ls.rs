use std::{fs, path::Path};

pub fn execute(path: Option<&str>) -> Result<(), String> {
    let target_path = path.unwrap_or(".");
    let path = Path::new(target_path);

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                if let Some(name) = entry.file_name().to_str() {
                    println!("{}", name);
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}
