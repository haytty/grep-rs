## Grep-rs

This project uses multi-threading in Rust to search a directory for lines matching a particular regular expression,
format them, and output them to the console.

### Installation

1. Clone the git repository:

```bash
git clone https://github.com/your_username/grep-rs.git
```

2. Move into the new directory:

```bash
cd grep-rs
```

3. Download dependencies:

```bash
cargo install --path .
```

### Usage

```
Usage: grep-rs [OPTIONS] <DIR> <REGEX>

Arguments:
  <DIR>    dir_path
  <REGEX>  regex

Options:
  -r, --recursive  
  -h, --help       Print help
  -V, --version    Print version
```
