# Sudoku
Homemade Sudoku solver in a variety of different languages and frameworks.

The samples folder has a variety of different test cases. Every binary takes one argument, the file path of the test cases.

## C
- Install `make` and `cc`
- `cd c`
- `make`
- `./main ../samples/all.txt`

## Rust
- Install rust and `cargo`
- `cd rust`
- `cargo build`
- `cargo run -- ../samples/all.txt`
- `cargo run -- ../samples/all.txt --debug` to show which puzzles failed

# Credits
- puzzles generated using [qqwing](https://qqwing.com)