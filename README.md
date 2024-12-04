# sudoku-checker

Checks whether a solved sudoku is a valid solution.

If it's incorrect, it prints what's wrong.

### Usage

This program takes an argument specifying a file where the input is stored.

The input file must contain 9 lines of 9 characters each. Each character must be a digit.

There is no checking or validation. The user must ensure that they're sending the correct file.

If you're using the Just runner, you may paste the file in `inputs/sudoku_input`, and run

```
just
```

Otherwise, you may run,

```
cargo run -- <file-path>
```

