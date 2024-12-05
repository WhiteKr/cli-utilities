use std::fs;

pub fn execute(files: &[String]) -> Result<(), String> {
    if files.is_empty() {
        return Err("No files specified".to_string());
    }

    for file in files {
        let content =
            fs::read_to_string(file).map_err(|e| format!("Failed to read '{}': {}", file, e))?;

        print!("{}", content);
    }

    Ok(())
}
