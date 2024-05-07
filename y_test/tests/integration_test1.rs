use y_test;

/*
tests目录下添加公共模块(多个integration tests共享)
    * 方式一(tests目录下创建文件夹common和common.rs) ===> 执行cargo test时,出现Running tests\common.rs(即使该文件不含任何测试函数)
    * 方式二(推荐)(tests目录下创建文件夹common,common目录下创建mod.rs) ===> 执行cargo test时,不显示common相关内容
*/
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, y_test::add_two(2));
}
