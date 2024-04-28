// Integration Tests

/*
Integration Tests目录位置:项目顶层目录(即与src目录同级)

tests目录下每个文件都是一个单独的crate(即导入lib.rs内容时,不同文件需要单独导入)

Integration Tests中不需要添加#[cfg(test)]注解(即只有运行Integration Tests时,对应文件才会被编译)

Integration Tests的输出包含单元测试(unit tests)、集成测试(integration test)和文档测试(doc tests),如果某个部分存在测试失败情况,则剩余部分不会被运行(example:如果单元测试存在测试失败情况,则集成测试和文档测试不会有任何输出)
*/
use y_test;

#[test]
fn t0() {
    assert_eq!(4, y_test::add_two(2));
}

#[test]
fn t1() {
    assert_eq!(5, y_test::add_two(3));
}
