use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::process;

// Function to delete the "node_modules" folder
fn delete_node_modules(folder_path: &Path) -> io::Result<()> {
    let node_modules_path = folder_path.join("node_modules");
    if node_modules_path.exists() {
        println!("Deleting {:?}", node_modules_path);
        fs::remove_dir_all(node_modules_path)?;
    }
    Ok(())
}

// Function to traverse the directory and delete "node_modules" folders
fn traverse_and_delete_node_modules(root_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(root_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            delete_node_modules(&entry_path)?;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <folder_name>", &args[0]);
        process::exit(1);
    }

    let folder_name = &args[1];
    let root_directory = Path::new(folder_name);

    if let Err(err) = traverse_and_delete_node_modules(root_directory) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

