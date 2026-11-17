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

#### 文档和测试

[文档](x_docs\cargo-doc_help.md)

[测试](x_tests\cargo-test_help.md)