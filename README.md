# rust_grep-lite
It outputs lines that contain a string pattern you want to search for from standard input or a file.
## Build
```
cargo build
or
cargo build --release
```
## Usage
```
Usage: rust_grep-lite --pattern <PATTERN> <--file <FILEPATH>|--stdin>

Options:
      --pattern <PATTERN>  The string pattern to search for each lines.
      --file <FILEPATH>    Input from file(specify file path).
      --stdin              Input from stdin.
  -h, --help               Print help
```