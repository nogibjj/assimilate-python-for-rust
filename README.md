# Assimilate Python for Rust

## Day 5:  Next time figure out why hello world won't work!!!


## Day 4:  Call Python from Rust

Ran into issues call Python from Rust.  Need to debug

```bash
@noahgift ➜ /workspaces/assimilate-python-for-rust/callPy (main) $ cargo build -j 16
   Compiling call_py v0.1.0 (/workspaces/assimilate-python-for-rust/callPy)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/tmp/rustcTLYgtY/symbols.o"
  ```


## Day 3:

A.  Ruff
Got Rust linter working called `ruff` and have a sample project in pyruff-example.
To test it out create a virtualenv, source it, then run `make install` then `make lint`
You should get the following

```bash
(.venv) @noahgift ➜ /workspaces/assimilate-python-for-rust/pyruff-example (main) $ make lint
ruff check . 
hello.py:3:8: F401 [*] `os` imported but unused
Found 1 error.
[*] 1 potentially fixable with the --fix option.
make: *** [Makefile:12: lint] Error 1
```

Example [docs for Ruff](https://beta.ruff.rs/docs/usage/)

B. [RustPython](https://github.com/RustPython/RustPython) playing around



## Day 2:  Setup a Rust and Python interaction

* https://github.com/PyO3/pyo3
* https://github.com/RustPython/RustPython


## Day 1:  Setup a Rust and Python interaction

`cargo new hello`
`cd hello`
`cargo run -- --help`
`cargo run -- marco --input "Marco"`
`./hello/target/debug/hello marco --input "Marco"`