# Rust Grep
Grep is a command-line utility in Unix and Unix-like operating systems used for searching and manipulating text or regular expressions within files. The name "grep" stands for `Global Regular Expression Print`.

This implementation is part of the exercise in chapter 11 of the 'Rust Programming Language' book.

## Usage
The program accepts two arguments:  
1. The string to search for, and
2. The file to search in

As an example,  
```sh
cargo run -- body poem.txt > output.txt
```
Will search for the string `body` in `poem.txt` and write the lines containing that string to output.txt.
