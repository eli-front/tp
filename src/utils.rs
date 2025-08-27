use crate::types::SavedDir;

pub fn get_saved_dirs() -> Vec<SavedDir> {
    let config = get_config();

    // split by new line
    let lines: Vec<&str> = config.split("\n").collect();

    // split by comma
    let saved_dirs: Vec<SavedDir> = lines
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() != 2 {
                None
            } else {
                Some(SavedDir {
                    name: parts[0].to_string(),
                    path: parts[1].to_string(),
                })
            }
        })
        .collect();

    saved_dirs
}

pub fn get_current_dir() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn save_dir(name: &str, path: &str) -> Result<(), String> {
    let saved_dirs = get_saved_dirs();
    let new_saved_dir = SavedDir {
        name: name.to_string(),
        path: path.to_string(),
    };

    // check if the name already exists
    let saved_dir_with_same_name = saved_dirs.iter().find(|dir| dir.name == name);

    if saved_dir_with_same_name.is_some() {
        return Err("Name already exists".to_string());
    }

    let new_saved_dirs = saved_dirs.iter().chain(std::iter::once(&new_saved_dir));

    let new_config = new_saved_dirs
        .map(|dir| format!("{},{}", dir.name, dir.path))
        .collect::<Vec<String>>()
        .join("\n");

    set_config(&new_config);

    Ok(())
}

pub fn set_dirs(dirs: Vec<SavedDir>) {
    let new_config = dirs
        .iter()
        .map(|dir| format!("{},{}", dir.name, dir.path))
        .collect::<Vec<String>>()
        .join("\n");
    set_config(&new_config);
}

fn get_config() -> String {
    // file located at ~/.tp/config
    let home = std::env::var("HOME").unwrap();

    let config_path = format!("{}/.tp/config", home);

    // if file does not exist, create it
    if !std::path::Path::new(&config_path).exists() {
        std::fs::create_dir_all(format!("{}/.tp", home)).unwrap();
        std::fs::write(&config_path, "").unwrap();
    }

    let config = std::fs::read_to_string(config_path).unwrap();

    config
}

fn set_config(config: &str) {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.tp/config", home);
    std::fs::write(config_path, config).unwrap();
}

pub fn remove_dir(name: &str) {
    let all_dirs = get_saved_dirs()
        .into_iter()
        .filter(|dir| dir.name != name)
        .collect();

    set_dirs(all_dirs);
}
