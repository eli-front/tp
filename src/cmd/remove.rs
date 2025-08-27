use crate::utils::remove_dir;

pub fn remove(name: &String) {
    println!("Remove command invoked with name: {:?}", name);
    let name = name.as_ref();
    remove_dir(name);
    println!("Directory removed successfully");
}
