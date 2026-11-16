### 目录

* crates/a_basic--->基础
* crates/b_basic--->基础
* crates/c_pattern_matching--->模式匹配
* crates/d_error_handling--->错误处理
* crates/e_container--->容器
* f_advanced--->高级
* g_concurrency--->多线程
* h_file_operate--->文件操作
* i_macro--->宏
* x_docs--->文档注释
* x_package_crate_module--->package、crate、module
* x_tests--->测试
* x_z_other--->语言其他有用学习
* y_serde_serde_json--->第三方库serde,serde_json学习

### cargo 常用命令

#### 创建项目

```shell
cargo new hello          # 新建binary项目
cargo new mylib --lib    # 新建library项目
```

#### 编译和运行

```shell
cargo check              # 只检查能否编译通过,速度最快,写代码时最常用
cargo build              # debug编译,产物在target/debug/
cargo build --release    # 优化编译,产物在target/release/
cargo run                # 编译并运行
cargo clean              # 删除target/目录

# workspace中用-p指定package
cargo check -p a_basic                   # 只检查a_basic包
cargo build -p a_basic                   # 只编译a_basic包
cargo run -p a_basic --bin comment_      # 运行a_basic包中src/bin/comment_.rs
```

#### 依赖管理

```shell
cargo add rand                       # 添加依赖
cargo add serde --features derive    # 添加依赖并开启某个 feature
cargo remove rand                    # 删除依赖
cargo update                         # 更新依赖版本(Cargo.lock)
cargo tree                           # 查看依赖树
```
