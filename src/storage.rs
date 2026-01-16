use std::fs;
use std::io;
use std::path::PathBuf;

use crate::data::AppData;

pub fn get_data_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("heat");
    data_dir.join("data.json")
}

pub fn load_data() -> io::Result<AppData> {
    let path = get_data_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = AppData::new();
        save_data(&data)?;
        return Ok(data);
    }

    let contents = fs::read_to_string(&path)?;
    let data: AppData = serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(data)
}

pub fn save_data(data: &AppData) -> io::Result<()> {
    let path = get_data_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let contents = serde_json::to_string_pretty(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(&path, contents)
}
