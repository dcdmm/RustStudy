### Ignoring Some Tests Unless Specifically Requested

Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`. Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the ignore attribute to exclude them, as shown here:

```rust
// Filename: src/lib.rs

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
running 1 test
test expensive_test ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests xxx

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The expensive_test function is listed as ignored. If we want to run only the ignored tests, we can use cargo test -- --ignored:

```text
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests xxx

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

By controlling which tests run, you can make sure your `cargo test` results will be fast. When you’re at a point where it makes sense to check the results of the ignored tests and you have time to wait for the results, you can run `cargo test -- --ignored` instead. If you want to run all tests whether they’re ignored or not, you can run `cargo test -- --include-ignored`.