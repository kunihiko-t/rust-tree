# rust-tree

rust-tree is a tree command for my Rust practice.

# Install 

cargo install --git https://github.com/kunihiko-t/rust-tree

# Uasge

use `rust-tree --help` for help

```
rust-tree 0.1
Kunihiko Tanaka

USAGE:
    rust-tree [FLAGS] [path]...

FLAGS:
    -a               All files are printed. By default tree does not print hidden files (those beginning with a dot
                     '.'). In no event does tree print the file system constructs '.' (current directory) and '..'
                     (previous directory).
    -d               List directories only.
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>...    root path
```