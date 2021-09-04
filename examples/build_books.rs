use std::process::Command;
use std::path::{Path, PathBuf};

fn main() {
    let dirs: Vec<PathBuf> = vec![
        ["examples", "figaro"].iter().collect(),
        ["examples", "torikaeshi"].iter().collect(),
    ];

    dirs.iter()
        .for_each(|dir| build_book(dir));
}

fn build_book(dir: &Path) {
    let status = Command::new("mdbook")
        .arg("build")
        .current_dir(dir)
        .status()
        .unwrap();
    println!("status = {:?}", status);
}
