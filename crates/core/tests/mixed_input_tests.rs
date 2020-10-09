use advent_of_code::solve;
use std::fs::{self, read_to_string, DirEntry};
use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[test]
fn mixed_input() -> Result<(), String> {
    let mut entries = Vec::new();
    visit_dirs(Path::new("src"), &mut |entry| {
        let path = entry.path();
        if let Some(path_string) = path.to_str() {
            if path_string.ends_with("_input.txt") {
                entries.push(entry.path());
            }
        }
    })
    .map_err(|_| "Unable to visit directories".to_string())?;
    for entry in entries {
        let entry_clone = entry.clone();
        let p = entry_clone.to_str().ok_or("Invalid utf-8 in entry")?;
        println!("{:?}", entry);
        let input_string = read_to_string(entry).map_err(|_| "Cannot read file")?;
        for year in 2018..=2019 {
            for day in 1..=25 {
                for part in 1..=2 {
                    println!("cargo run -q {} {} {} < {}", year, day, part, p);
                    let _ = solve(year, day, part, &input_string);
                }
            }
        }
    }

    Ok(())
}
