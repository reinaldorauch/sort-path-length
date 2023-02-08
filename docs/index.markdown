---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: default
---

Here is the documentation

## Installation

```bash
cargo install sort-path-length
```

## Usage

```bash
$ sort-path-length <path>
```

where `<path>` is the path of a file that contains a list of paths separated by `\n` (newline)

### example input file

```
/a/absolute/path
/a/b/c/d/e
/a
/a/dpasodj
```

the output on stdout should be

```
/a
/a/dpasodj
/a/absolute/path
/a/b/c/d/e
```


## License

GPLv3

## TODO

* Accept input from stdin
* Add option to change sort order
* Check usage on other systems (eg: Windows)
* Improve documentation