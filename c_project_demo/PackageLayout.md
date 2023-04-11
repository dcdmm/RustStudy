```markdown
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

* Cargo.toml and Cargo.lock are stored in the root of your package (package root).
* Source code goes in the src directory.
* The default library file is src/lib.rs.
* The default executable file is src/main.rs.
  * Other executables can be placed in src/bin/.
* Benchmarks go in the benches directory.
* Examples go in the examples directory.
* Integration tests go in the tests directory.


If a binary, example, bench, or integration test consists of multiple source files, place a main.rs file along with the extra modules within a subdirectory of the src/bin, examples, benches, or tests directory. The name of the executable will be the directory name.

***

A crate can come in one of two forms: a binary crate or a library crate.
Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.
Each must have a function called main that defines what happens when the executable runs.

Library crates don’t have a main function, and they don’t compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.
Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
Cargo passes the crate root files to rustc to build the library or binary.

A package can contain as many binary crates as you like, but at most only one library crate.
A package must contain at least one crate, whether that’s a library or binary crate.

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.