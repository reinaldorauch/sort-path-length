use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // read file line paths
    if env::args().len() != 2 {
        panic!("Usage: <path>");
    }

    let args: Vec<String> = env::args().collect();
    let file_to_read = &args[1];

    let file = fs::read_to_string(file_to_read).unwrap();

    let mut path_hash_map: HashMap<u32, Vec<&str>> = HashMap::new();

    for line_path in file.lines() {
        let components = Path::new(line_path).components().collect::<Vec<_>>();
        path_hash_map
            .entry(components.len() as u32)
            .and_modify(|v| v.push(line_path))
            .or_insert(vec![line_path]);
    }

    let mut lengths = path_hash_map.keys().collect::<Vec<_>>();

    lengths.sort();

    for k in lengths {
        match path_hash_map.get(k) {
            Some(p) => {
                let mut paths = p.clone();
                paths.sort();
                println!("{}", paths.join("\n"));
            }
            None => {
                // noop
            }
        };
    }
}
