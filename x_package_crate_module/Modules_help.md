A module is a container for zero or more items.

A module item is a module, surrounded in braces, named, and prefixed with the keyword mod. A module item introduces a new, named module into the tree of modules making up a crate. Modules can nest arbitrarily.

### Module Source Filenames
A module without a body is loaded from an external file.

* For a module named util declared in the crate root(`mod util;`), the compiler will look for the module’s code in:
  * 方式一(参考:file_path_style1/arithmetic0_.rs): src/util.rs
  * 方式二(参考:file_path_style2/mod.rs): src/util/mod.rs
* For a module named util_sub that is a submodule of util(`mod util_sub;`), the compiler will look for the module’s code in:
  * 方式一: src/util/util_sub.rs
  * 方式二(参考:file_path_style2/file_path_style2_sub/mod.rs): src/util/util_sub/mod.rs

注意:同一个模块方式一、方式二不能同时存在


#### 方式一示例

以lib.rs为例,编译器从lib.rs开始,每遇到一个`mod xxx;`就引入对应文件作为子模块(如`mod util;`引入util.rs),再在该文件中继续查找`mod`,逐层向下,最终所有文件组成一棵模块树一起编译

```text
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── util.rs
│   └── util
│      ├──util_xxx.rs
│      ├──util_sub.rs
│      ├──util_sub
│         ├──util_sub_xxx.rs
```

| Module Path                         | Filesystem Path                   | File Contents               |
| ----------------------------------- | --------------------------------- | --------------------------- |
| crate                               | src/lib.rs                        | mod util;                   |
| crate                               | src/main.rs                       | mod util;                   |
| crate::util                         | src/util.rs                       | mod util_xxx; mod util_sub; |
| crate::util::util_xxx               | src/util/util_xxx.rs              |                             |
| crate::util::util_sub               | src/util/util_sub.rs              | mod util_sub_xxx;           |
| crate::util::util_sub::util_sub_xxx | src/util/util_sub/util_sub_xxx.rs |                             |

#### 方式二示例

```text
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── util
│      ├──mod.rs
│      ├──util_xxx.rs
│      ├──util_sub
│         ├──mod.rs
│         ├──util_sub_xxx.rs
```

| Module Path                         | Filesystem Path                   | File Contents               |
| ----------------------------------- | --------------------------------- | --------------------------- |
| crate                               | src/lib.rs                        | mod util;                   |
| crate                               | src/main.rs                       | mod util;                   |
| crate::util                         | src/util/mod.rs                   | mod util_xxx; mod util_sub; |
| crate::util::util_xxx               | src/util/util_xxx.rs              |                             |
| crate::util::util_sub               | src/util/util_sub/mod.rs          | mod util_sub_xxx;           |
| crate::util::util_sub::util_sub_xxx | src/util/util_sub/util_sub_xxx.rs |                             |