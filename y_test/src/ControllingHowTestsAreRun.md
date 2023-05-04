## Running Tests in Parallel or Consecutively

When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster and you get feedback quicker. Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

For example, say each of your tests runs some code that creates a file on disk named test-output.txt and writes some data to that file. Then each test reads the data in that file and asserts that the file contains a particular value, which is different in each test. Because the tests run at the same time, one test might overwrite the file in the time between another test writing and reading the file. The second test will then fail, not because the code is incorrect but because the tests have interfered with each other while running in parallel. One solution is to make sure each test writes to a different file; another solution is to run the tests one at a time.

If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary. Take a look at the following example:

```shell
cargo test -- --test-threads=1
```

We set the number of test threads to 1, telling the program not to use any parallelism. Running the tests using one thread will take longer than running them in parallel, but the tests won’t interfere with each other if they share state.


## Showing Function Output

By default, if a test passes, Rust’s test library captures anything printed to standard output. For example, if we call println! in a test and the test passes, we won’t see the println! output in the terminal; we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.

As an example, Listing 1 has a silly function that prints the value of its parameter and returns 10, as well as a test that passes and a test that fails.

Filename: src/lib.rs

```rust
// Listing 1: Tests for a function that calls println!

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

When we run these tests with `cargo test`, we’ll see the following output:

```text
$ cargo test
   Compiling y_test v0.1.0 (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\y_test)
warning: function `prints_and_returns_10` is never used
 --> y_test\src\lib.rs:3:4
  |
3 | fn prints_and_returns_10(a: i32) -> i32 {
  |    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `y_test` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests src\lib.rs (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\target\debug\deps\y_test-7c884fa28fc8240d.exe)

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', y_test\src\lib.rs:21:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

Note that nowhere in this output do we see I got the value 4, which is what is printed when the test that passes runs. That output has been captured. The output from the test that failed, I got the value 8, appears in the section of the test summary output, which also shows the cause of the test failure.

If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with --show-output.

```shell
cargo test -- --show-output
```

When we run the tests in Listing 1 again with the --show-output flag, we see the following output:

```text
$ cargo test -- --show-output
warning: function `prints_and_returns_10` is never used
 --> y_test\src\lib.rs:3:4
  |
3 | fn prints_and_returns_10(a: i32) -> i32 {
  |    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `y_test` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src\lib.rs (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\target\debug\deps\y_test-7c884fa28fc8240d.exe)

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', y_test\src\lib.rs:21:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

## Ignoring Some Tests Unless Specifically Requested

Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`. Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the ignore attribute to exclude them, as shown here:

Filename: src/lib.rs

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

After #[test] we add the #[ignore] line to the test we want to exclude. Now when we run our tests, it_works runs, but expensive_test doesn’t:

```text
$ cargo test
   Compiling y_test v0.1.0 (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\y_test)
    Finished test [unoptimized + debuginfo] target(s) in 0.80s
     Running unittests src\lib.rs (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\target\debug\deps\y_test-7c884fa28fc8240d.exe)

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The expensive_test function is listed as ignored. If we want to run only the ignored tests, we can use cargo test -- --ignored:

```text
$ cargo test -- --ignored
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src\lib.rs (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\target\debug\deps\y_test-7c884fa28fc8240d.exe)

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running unittests src\main.rs (C:\Users\dcdmm\Music\GitHubProjects\RustStudy\target\debug\deps\y_test-b52f5cf221bb84d0.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests y_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

By controlling which tests run, you can make sure your `cargo test` results will be fast. When you’re at a point where it makes sense to check the results of the ignored tests and you have time to wait for the results, you can run `cargo test -- --ignored` instead. If you want to run all tests whether they’re ignored or not, you can run `cargo test -- --include-ignored`.