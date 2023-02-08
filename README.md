# sort-path-length

Binary package to sort paths based on their component's length


## Instalation

`$ cargo install sort-path-length`

## Usage

`$ sort-path-length <path>`

where `<path>` is a file containing paths separtated by `\n` (new line).

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



## TODO

* Accept input from stdin
* Add option to change sort order
* Check usage on other systems (eg: Windows)
* Improve documentation