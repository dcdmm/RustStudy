There are two styles of tests within a Cargo project:

* Unit tests which are functions marked with the #[test] attribute located within your library or binaries (or any target enabled with the test field). These tests have access to private APIs located within the target they are defined in.

* Integration tests which is a separate executable binary, also containing `#[test]` functions, which is linked with the project’s library and has access to its public API.
  * Files located under the tests(项目顶层目录,src目录同级) directory are integration test.
  * integration test不需要添加#[cfg(test)](integration test仅在被测试时,才会被编译和执行)
  * 存在integration tests时执行cargo test,The three sections of output include the unit tests, the integration test, and the doc tests. Note that if any test in a section fails, the following sections will not be run. For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
  
### Target Selection

When no target selection options are given, cargo test will build the following targets of the selected packages:

* lib — used to link with binaries, examples, integration tests, and doc tests
* bins (only if integration tests are built and required features are available)
* examples — to ensure they compile
* lib as a unit test
* bins as unit tests
* integration tests
* doc tests for the lib target

#### 示例

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
cargo test --bin y_test -- tests_0  # 会匹配main.rs中所有包含tests_0的项
```

* 测试src/main.rs tests_0模块下函数main_test0_t0
```shell
cargo test --bin y_test -- tests_0::main_test0_t0  # 会匹配main.rs tests_0模块下所有包含main_test0_t0的项
```

* 测试src/main.rs tests_0函数t0
```shell
cargo test --bin y_test -- t0  # 会匹配mian.rs中所有包含t0的项
```

* 测试src/bin/bin_test.rs
```shell
cargo test --bin bin_test
```

* Test all binary targets.
```shell
cargo test --bins
```

* 测试tests/integration_test0.rs
```shell
cargo test --test integration_test0
```

* 测试tests/integration_test0.rs t0函数
```shell
cargo test --test integration_test0 -- t0 # 会匹配integration_test0.rs所有包含t0的项
```

* 测试src/normal_test.rs
  * main.rs ===> mod normal_test;
     ```shell
     cargo test --bin y_test -- normal_test # y_test为包名
     ```
   * lib.rs ===> mod normal_test;
     ```shell
     cargo test --lib -- normal_test
     ``` 

