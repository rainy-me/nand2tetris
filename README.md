# About

Project files related to [Build a Modern Computer from First Principles: From Nand to Tetris](https://www.coursera.org/learn/build-a-computer/)

## To run part I test script

```sh
deno run --allow-read --allow-run --unstable index.ts --p <TASK_DIR>
```

it will run all `.asm` / `.hdl` related tests under `<TASK_DIR>` and watch changes to rerun.

## To run part II cli and tests

```sh
# to assemble a .asm file
cargo run assemble <TASK_DIR>

# to translate a .vm file
cargo run translate <TASK_DIR>

# to run tests
cargo test
```

it will run all project related tests.
