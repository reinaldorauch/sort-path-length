# sort-path-length

Binary package to sort paths based on their component's length


## Instalation

`$ cargo install sort-path-length`

## Usage

`$ sort-path-length <path>`

where `<path>` is a file containing paths separtated by `\n` (new line).

Or you can pass via pipe in the terminal like so:

`$ cat a_file | sort-path-length`

If you do not pass a file in the argument or the file name is "-" the program will expect input in the stdio

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