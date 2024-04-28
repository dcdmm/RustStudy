When no target selection options are given, cargo test will build the following targets of the selected packages:

* lib — used to link with binaries, examples, integration tests, and doc tests
* bins (only if integration tests are built and required features are available)
* examples — to ensure they compile
* lib as a unit test
* bins as unit tests
* integration tests
* doc tests for the lib target

***

* 测试src/lib.rs
```shell
cargo test --lib
```

* 测试src/main.rs
```shell
cargo test --bin y_test # y_test为包名
```

* src/bin/bin_test.rs
```shell
cargo test --bin bin_test
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