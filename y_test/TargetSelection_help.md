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

* 测试src/main.rs tests_0模块
```shell
cargo test --bin y_test -- tests_0  # y_test为包名
```

* 测试src/main.rs tests_0模块下函数main_test0_t0
```shell
cargo test --bin y_test -- tests_0::main_test0_t0  # y_test为包名
```

* 测试src/main.rs tests_0函数t0
```shell
cargo test --bin y_test -- t0 --exact  # y_test为包名
```

* 测试src/bin/bin_test.rs
```shell
cargo test --bin bin_test
```

* Test all binary targets.
```shell
cargo test --bins
```

<!-- * tests/integration_test0.rs
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
        ``` -->

