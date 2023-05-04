When no target selection options are given, cargo test will build the following targets of the selected packages:

* lib — used to link with binaries, examples, integration tests, and doc tests
* bins (only if integration tests are built and required features are available)
* examples — to ensure they compile
* lib as a unit test
* bins as unit tests
* integration tests
* doc tests for the lib target

***

* src/lib.rs
```shell
cargo test --lib
```

* src/main.rs
```shell
cargo test --bin y_test
```

* src/bin/example0.rs
```shell
cargo test --bin example0
```

* tests/integration_test0.rs
```shell
cargo test --test integration_test0
```

* src/ordinary.rs
  * mod ordinary;  ===>mainrs
        ```shell
        cargo test --bin y_test
        ```
   * mod ordinary; ===>lib.rs
        ```shell
        cargo test --lib
        ```

* Test all binary targets.
```shell
cargo test --bins
```