use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::str;

///
/// It reads the input as lines and parses each one as a path and then sorts them
/// out by the length of their path components.
/// Pass invert as true to invert the order
///
pub fn sort_path_length(input: &str, invert: bool, output: &mut dyn Write) {
    let mut path_hash_map: HashMap<usize, Vec<&str>> = HashMap::new();

    for line_path in input.lines() {
        let components = Path::new(line_path).components().collect::<Vec<_>>();
        path_hash_map
            .entry(components.len())
            .and_modify(|v| {
                // Addding to the inner list of same length components in ordered manner
                if invert {
                    for (index, &l) in v.iter().enumerate() {
                        if line_path.partial_cmp(l).unwrap() == Ordering::Greater {
                            v.insert(index, line_path);
                            break;
                        }
                    }
                } else {
                    for (index, &l) in v.iter().enumerate() {
                        if line_path.partial_cmp(l).unwrap() == Ordering::Less {
                            v.insert(index, line_path);
                            // using a early return or shortcut to not use a
                            // flag to check if the element is inserted after
                            // the current elements loop
                            return;
                        }
                    }

                    v.push(line_path);
                }
            })
            .or_insert(vec![line_path]);
    }

    let mut lengths = path_hash_map.keys().collect::<Vec<_>>();

    if invert {
        lengths.sort_by(|a, b| b.partial_cmp(a).unwrap());
    } else {
        lengths.sort();
    }

    // Writing to output in order
    for k in lengths {
        if let Some(paths) = path_hash_map.get(k) {
            for &p in paths {
                output.write(p.as_bytes()).unwrap();
                output.write("\n".as_bytes()).unwrap();
            }
        };
    }
}

#[test]
pub fn test_default_sort_order() {
    let input = "/a/absolute/path\n/a/b/c/d/e\n/a\n/a/dpasodj";

    let mut output: Vec<u8> = Vec::new();

    sort_path_length(input, false, &mut output);

    let expected = "/a\n/a/dpasodj\n/a/absolute/path\n/a/b/c/d/e\n";

    assert_eq!(expected, str::from_utf8(&output).unwrap());
}

#[test]
pub fn test_inverted_sort_order() {
    let input = "/a/absolute/path\n/a/b/c/d/e\n/a\n/a/dpasodj";

    let mut output: Vec<u8> = Vec::new();

    sort_path_length(input, true, &mut output);

    let expected = "/a/b/c/d/e\n/a/absolute/path\n/a/dpasodj\n/a\n";

    assert_eq!(expected, str::from_utf8(&output).unwrap());
}

#[test]
pub fn test_default_sort_order_same_length_paths() {
    let input = "/a/b/c/d/e
/b/c/d/e/f
/c/d/e/f/g
/a/e/d/c/b
/b/f/e/c/d
";

    let mut output: Vec<u8> = Vec::new();

    sort_path_length(input, false, &mut output);

    let expected = "/a/b/c/d/e
/a/e/d/c/b
/b/c/d/e/f
/b/f/e/c/d
/c/d/e/f/g
";

    assert_eq!(expected, str::from_utf8(&output).unwrap());
}

#[test]
pub fn test_inverted_sort_order_same_length_paths() {
    let input = "/a/b/c/d/e
/b/c/d/e/f
/c/d/e/f/g
/a/e/d/c/b
/b/f/e/c/d
";

    let mut output: Vec<u8> = Vec::new();
    sort_path_length(input, true, &mut output);

    let expected = "/c/d/e/f/g
/b/f/e/c/d
/b/c/d/e/f
/a/e/d/c/b
/a/b/c/d/e
";

    assert_eq!(expected, str::from_utf8(&output).unwrap());
}
