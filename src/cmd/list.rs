pub fn list() {
    let dirs = crate::utils::get_saved_dirs();

    if dirs.is_empty() {
        println!("No directories saved yet.");
        return;
    }

    println!("Available directories are:");
    for dir in dirs {
        println!(" - {}", dir.name);
    }
}
