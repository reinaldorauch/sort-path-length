use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;

///
/// It reads the input as lines and parses each one as a path and then sorts them
/// out by the length of their path components.
/// Pass invert as true to invert the order
///
pub fn sort_path_length(input: String, invert: bool) -> String {
    let mut path_hash_map: HashMap<usize, Vec<&str>> = HashMap::new();

    for line_path in input.lines() {
        let components = Path::new(line_path).components().collect::<Vec<_>>();
        path_hash_map
            .entry(components.len())
            .and_modify(|v| v.push(line_path))
            .or_insert(vec![line_path]);
    }

    let mut lengths = path_hash_map.keys().collect::<Vec<_>>();

    if invert {
        lengths.sort_by(|a, b| b.partial_cmp(a).unwrap());
    } else {
        lengths.sort();
    }

    let mut output: Vec<&str> = Vec::new();

    for k in lengths {
        match path_hash_map.get(k) {
            Some(p) => {
                let mut paths = p.clone();

                if invert {
                    paths.sort_by(|a, b| b.partial_cmp(a).unwrap());
                } else {
                    paths.sort();
                }

                output.append(&mut paths);
            }
            None => {
                // noop
            }
        };
    }

    output.iter().fold(String::new(), |mut o, p| {
        o.write_str(p).unwrap();
        o.write_char('\n').unwrap();
        o
    })
}

#[test]
pub fn test_default_sort_order() {
    let input = "/a/absolute/path\n/a/b/c/d/e\n/a\n/a/dpasodj";

    let output = sort_path_length(input.to_string(), false);

    let expected = "/a\n/a/dpasodj\n/a/absolute/path\n/a/b/c/d/e\n";

    assert_eq!(expected, output);
}

#[test]
pub fn test_inverted_sort_order() {
    let input = "/a/absolute/path\n/a/b/c/d/e\n/a\n/a/dpasodj";

    let output = sort_path_length(input.to_string(), true);

    let expected = "/a/b/c/d/e\n/a/absolute/path\n/a/dpasodj\n/a\n";

    assert_eq!(expected, output);
}
