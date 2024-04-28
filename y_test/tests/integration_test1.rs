use y_test;

/*
integration test添加公共模块(多个测试函数使用时)
* 方式一
    * tests目录下创建文件夹common和common.rs
    * 执行cargo test时,显示Running tests\common.rs,即使该文件不含任何测试函数(tests目录下每个文件都是一个单独的crate)
* 方式二(推荐)
    * tests目录下创建文件夹comon,comon目录下创建文件mod.rs
    * 执行cargo test时,不会显示Running tests\common.rs
*/
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, y_test::add_two(2));
}
