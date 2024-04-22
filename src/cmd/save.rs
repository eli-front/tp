use crate::utils::save_dir;

pub fn save(current_dir: &String, name: &Option<String>) {
    println!(
        "Save command invoked with name: {:?} and current_dir: {}",
        name, current_dir
    );

    let dir_name = name
        .as_ref()
        .unwrap_or(&current_dir)
        .split("/")
        .last()
        .unwrap()
        .to_string();

    // if name is None set name to current dir name
    let name = name.as_ref().unwrap_or(&dir_name);

    let r = save_dir(&name, current_dir);

    match r {
        Ok(()) => println!("Directory saved successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
