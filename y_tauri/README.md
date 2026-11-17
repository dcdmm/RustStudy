# y_tauri

Tauri 2 + React 19 + TypeScript 7 + Vite 8 + React Router 8 入门案例,另外包含两个独立的Rust库:

1. `crates/cpu_tasks`: CPU密集型任务,学习rayon并行计算
2. `crates/io_tasks`: IO密集型任务,学习tokio异步并发

Tauri: 用Web技术(HTML/CSS/JS)写界面,用Rust写后端逻辑,打包成桌面应用。界面由系统自带的WebView渲染(Windows上是WebView2),不像Electron那样内置一个Chromium,所以安装包很小

参考文档: https://tauri.app

### 运行

```shell
cd y_tauri
npm install                          # 首次运行前安装前端依赖
npm run tauri dev                    # 开发模式,首次需要编译Rust,比较慢
npm run tauri build                  # 打包安装包,产物在target/release/bundle/
npm run tauri build -- --no-bundle   # 只编译exe,不生成安装包,产物在target/release/y_tauri.exe

npm run build                        # 前端类型检查(tsc -b) + 打包
npm run lint                         # 前端代码检查(oxlint)
```

Rust部分是一个Cargo workspace(`y_tauri/Cargo.toml`),在`y_tauri`目录下执行:

```shell
cargo test --workspace                                 # 测试所有Rust代码
cargo clippy --workspace --all-targets                 # Rust代码检查
cargo test -p cpu_tasks                                # 只测试cpu_tasks,不需要编译Tauri,几秒就能跑完
cargo test -p io_tasks                                 # 只测试io_tasks,测试用本地HTTP服务器,不需要外网
cargo run -p cpu_tasks --example cpu_bench --release   # 不启动应用,对比单线程和rayon并行的耗时
cargo run -p io_tasks --example check_urls             # 不启动应用,对比顺序请求和并发请求的耗时(需要外网)
```

`npm run tauri dev`做了什么:

1. 执行tauri.conf.json中的`beforeDevCommand`(`npm run dev`),启动Vite开发服务器: http://localhost:1420
2. 用cargo编译并运行src-tauri
3. 打开窗口,加载`devUrl`(就是上面的Vite地址)
4. 修改前端代码 -> 页面热更新;修改Rust代码(包括`crates/`下的库) -> 自动重新编译并重启应用

`npm run tauri build`做了什么:

1. 执行`beforeBuildCommand`(`npm run build`),把前端打包到dist/
2. release模式编译Rust,并把`frontendDist`(dist/)嵌入到exe中,运行时不再需要Vite
3. 生成安装包(msi/nsis)

### 界面

```
┌────────────┬──────────────────────────────────┐
│ 左侧菜单    │ 页面标题 + 说明 + 对应源码         │
│  首页       │                                  │
│  基础       │  ┌ Card ───────────────────────┐ │
│   1. ...    │  │ 示例                         │ │
│  通信       │  └─────────────────────────────┘ │
│   4. ...    │  ┌ Card ───────────────────────┐ │
│  进阶       │  │ 要点                         │ │
│   6. ...    │  └─────────────────────────────┘ │
│  性能       │                                  │
│   8. ...    │                                  │
├────────────┴──────────────────────────────────┤
│ 状态栏: 应用名 + 版本              已运行 N 秒   │
└───────────────────────────────────────────────┘
```

### 目录结构

```
y_tauri/
├── Cargo.toml                    # Cargo workspace: 成员是src-tauri和crates/*,统一管理依赖版本
├── target/                       # 所有Rust成员共用的编译产物目录
├── index.html                    # 前端入口HTML
├── package.json                  # 前端依赖和npm脚本
├── vite.config.ts                # Vite配置(固定端口1420,供Tauri加载)
├── tsconfig*.json                # TS配置: app(src/)和node(vite.config.ts)分开;@/指向src/
├── .oxlintrc.json                # oxlint配置,包括依赖方向检查
├── src/                          # 前端(React + TypeScript),features架构
│   ├── main.tsx                  # 入口: 挂载React和路由
│   ├── app/                      # 应用骨架: 把各个feature组装起来
│   │   ├── router.ts             # 创建路由,所有页面嵌套在MainLayout中
│   │   ├── routes.ts             # 页面配置,路由和左侧菜单都根据它生成
│   │   └── layout/               # MainLayout(布局)、Sidebar(左侧菜单)、StatusBar(状态栏)
│   ├── features/                 # 一个知识点一个文件夹,与Rust端的模块一一对应
│   │   ├── home/                 # HomePage.tsx + api.ts    <-> commands/system.rs
│   │   ├── command/              # CommandPage.tsx + api.ts <-> commands/greet.rs
│   │   ├── error-handling/       # ...                      <-> commands/math.rs
│   │   ├── state/                # ...                      <-> commands/counter.rs
│   │   ├── channel/              # ...                      <-> commands/countdown.rs
│   │   ├── event/                # EventPage.tsx            <-> uptime.rs
│   │   ├── window/               # 只有页面,直接用Tauri内置的窗口API
│   │   ├── plugin/               # 只有页面,直接用opener插件的API
│   │   ├── cpu/                  # CpuPage.tsx + 4个Card组件 <-> commands/cpu.rs -> crates/cpu_tasks
│   │   └── io/                   # IoPage.tsx + api.ts      <-> commands/io.rs  -> crates/io_tasks
│   └── shared/                   # 多个feature共用的代码
│       ├── components/           # Card、PageHeader
│       ├── hooks/                # useTauriEvent: 监听事件,组件卸载时自动取消
│       ├── lib/attempt.ts        # 把Promise转换为{ ok, data | error },配合useActionState使用
│       ├── tauri/events.ts       # 事件名 -> payload类型(状态栏和event页面都用)
│       └── styles/global.css     # 全局样式;组件样式用同目录的*.module.css(CSS Modules)
├── src-tauri/                    # Tauri应用的Rust后端
│   ├── Cargo.toml
│   ├── build.rs                  # 编译期处理tauri.conf.json、capabilities等
│   ├── tauri.conf.json           # Tauri核心配置: 窗口、前端地址、CSP、打包
│   ├── capabilities/
│   │   └── default.json          # 权限: 前端能调用哪些Tauri API
│   ├── icons/                    # 应用图标
│   └── src/
│       ├── main.rs               # 桌面端入口,只调用lib.rs中的run()
│       ├── lib.rs                # 组装应用: 插件、托管状态、setup、注册command
│       ├── error.rs              # 统一错误类型AppError、参数范围检查
│       ├── uptime.rs             # 后台任务,广播事件
│       └── commands/             # 按功能拆分的command,cpu.rs和io.rs负责调用crates中的库
└── crates/                       # 独立的Rust库,不依赖Tauri,可以单独测试和使用
    ├── cpu_tasks/                # rayon: 同步的纯计算代码
    │   ├── src/
    │   │   ├── primes.rs         # 并行迭代器par_iter: 统计质数
    │   │   ├── word_freq.rs      # fold + reduce: 词频统计
    │   │   ├── sort.rs           # rayon::join: 并行快速排序
    │   │   ├── pool.rs           # ThreadPoolBuilder: 指定线程数
    │   │   └── sample.rs         # 生成测试数据
    │   └── examples/cpu_bench.rs
    └── io_tasks/                 # tokio + reqwest: 异步网络请求
        ├── src/url_check.rs      # async/await、timeout、JoinSet、Semaphore
        └── examples/check_urls.rs
```

### 架构

前端按feature组织(实际项目中feature是业务模块,如笔记、设置;这里每个知识点当作一个feature):

1. 依赖方向: `app -> features -> shared`,只能从左往右引用
   - feature之间不能互相引用,多个feature都要用的代码放到`shared/`
   - `shared/`不能引用`features/`和`app/`
   - 由oxlint的`no-restricted-imports`规则检查(`.oxlintrc.json`),违反时`npm run lint`报错
2. import路径: 同一个feature(或同一层)内用相对路径`./api.ts`,跨层用别名`@/shared/...`
3. 每个feature调用Rust的代码放在自己的`api.ts`中,页面不直接写`invoke`,command名和类型只在一处定义
4. Tauri内置API(如`@tauri-apps/api/window`)和插件API本身就带类型,页面中直接调用
5. 新增一个知识点: 在`features/`下加一个文件夹,在`app/routes.ts`中加一项,Rust端在`commands/`下加一个文件

Rust端分层,依赖方向: `src-tauri -> crates/*`:

1. `crates/*`: 业务逻辑和计算,纯Rust,不依赖Tauri;可以单独测试,也可以被命令行工具等其他程序复用
2. `src-tauri/src/commands/`: 很薄的一层适配,负责检查参数、调用crates、转换错误、返回给前端
3. 计算量小的逻辑可以直接写在command文件中(如`counter.rs`),写成普通函数/方法,同样可以直接单元测试

### 页面与知识点

建议按左侧菜单顺序学习,每个页面顶部列出了对应的源码

| 菜单 | 前端(src/features/) | Rust | 知识点 |
| --- | --- | --- | --- |
| 首页 | `home/` | `commands/system.rs` | Tauri内置JS API vs 自定义command;command返回结构体 |
| 1. Command | `command/` | `commands/greet.rs` | `#[tauri::command]`、`invoke`、参数和返回值 |
| 2. 错误处理 | `error-handling/` | `commands/math.rs`、`error.rs` | `thiserror`自定义错误,`Err`在前端变为Promise reject |
| 3. 状态管理 | `state/` | `commands/counter.rs` | `.manage()`托管状态,`State<'_, T>`自动注入 |
| 4. Channel | `channel/` | `commands/countdown.rs` | `async` command,`Channel`推送进度;serde枚举 -> TS联合类型 |
| 5. 事件 | `event/` | `uptime.rs` | `setup`中启动后台任务,`emit`广播,多个`listen`同时收到 |
| 6. 窗口与权限 | `window/` | 无 | 窗口API;capabilities权限,未授权的调用被拒绝 |
| 7. 插件 | `plugin/` | `lib.rs` | `npm run tauri add`添加插件;权限范围(scope) |
| 8. CPU密集 | `cpu/` | `commands/cpu.rs`、`crates/cpu_tasks` | rayon: `par_iter`、`fold`+`reduce`、`join`、`ThreadPool`;单线程 vs 并行耗时对比 |
| 9. IO密集 | `io/` | `commands/io.rs`、`crates/io_tasks` | tokio: `JoinSet`、`Semaphore`、`timeout`;顺序 vs 并发耗时对比 |

### CPU密集 vs IO密集

| | CPU密集(cpu_tasks) | IO密集(io_tasks) |
| --- | --- | --- |
| 时间花在 | 计算 | 等待(网络、磁盘) |
| 提速方式 | 多个CPU核心同时计算 | 等待时去做别的事 |
| 工具 | rayon(线程池 + 工作窃取) | tokio(async/await + 事件循环) |
| 线程数 | 等于CPU核心数才能跑满 | 少量线程就能处理大量并发请求,单线程runtime也能并发 |
| 代码风格 | 同步函数 | async函数 |
| 在Tauri中调用 | `rayon::spawn` + `oneshot` channel,把结果交回async command | async command中直接`.await` |

为什么CPU密集任务不直接在async command中计算: async command运行在tokio的线程上,长时间计算会占住线程,影响其他异步任务;tokio文档建议把CPU密集任务交给rayon这样的专用线程池(见`commands/cpu.rs`中的`run_on_rayon`)

### 前端和Rust的3种通信方式

```
前端(WebView)                                          Rust
greet(name)                 ─── invoke(JSON) ───▶   #[tauri::command] fn greet
                            ◀── 返回值(JSON) ────
countdown(5, onEvent)       ─── invoke + Channel ─▶  async fn countdown(.., on_event: Channel<T>)
            onEvent(msg)    ◀── channel.send ×N ──
useTauriEvent("uptime", cb) ◀────── emit ────────   app.emit("uptime", payload)
```

| 方式 | 方向 | 适用场景 |
| --- | --- | --- |
| command | 前端 -> Rust -> 前端 | 一问一答(类似HTTP请求) |
| Channel | Rust -> 前端 | 一次command调用过程中的多条消息,如下载进度、网址检测结果(有序、速度快) |
| 事件 | Rust -> 所有监听者 | 与command无关,后端主动广播,如状态变化通知 |

### 注意点

1. 新写的command必须加到`generate_handler![]`中,否则前端调用报错: Command xxx not found
2. command名在前端保持snake_case;参数名在前端要写成camelCase(`total_seconds` -> `totalSeconds`)
3. 参数和返回值通过JSON传递,所以参数类型要实现`Deserialize`,返回值类型要实现`Serialize`;结构体加`#[serde(rename_all = "camelCase")]`让前端拿到camelCase字段
4. `State`、`AppHandle`等特殊参数由Tauri自动注入,前端不需要传;async command的参数中有引用(如`State<'_, T>`)时,返回值必须是`Result`
5. 普通command在主线程执行,耗时操作要用`async fn`,否则界面会卡住;CPU密集计算再交给rayon
6. 前端传来的参数不可信: 计算量、数据量等参数要在Rust端检查范围(`error.rs`中的`ensure_range`)
7. Rust和TS两边的类型目前是手动对应的(各feature的`api.ts`、`shared/tauri/events.ts`),修改Rust时要同步修改;也可以用[tauri-specta](https://github.com/specta-rs/tauri-specta)自动生成TS类型(目前还是rc版本)
8. `invoke`只能在Tauri窗口中使用,直接用浏览器打开 http://localhost:1420 会报错
9. 权限(`capabilities/default.json`):
   - `core:default`: 核心API的默认权限,主要是读取类(获取窗口标题、监听事件等),修改类(如`setTitle`)需要单独授权
   - 插件的权限: 添加插件时自动加上默认权限(如`opener:default`),权限还可以限制参数范围(scope)
   - 自己写的command默认都允许调用,不用在这里声明
   - 没有授权时,开发模式的报错会提示需要哪个权限;release模式只显示`not allowed by ACL`
10. CSP(`tauri.conf.json`): 打包后页面只能加载自身资源、只能连接Tauri的IPC;开发模式下页面直接由Vite提供,CSP不生效;Rust发出的网络请求不受CSP和CORS限制
11. 路由用的是`createBrowserRouter`,在子页面按F5刷新时,Tauri会返回index.html,由前端路由接管

Cargo workspace相关:

1. `[profile.*]`只能写在workspace根目录的`Cargo.toml`中,写在成员中不生效
2. `rust-version`不能低于依赖的要求: `resolver = "3"`会按`rust-version`选择依赖版本,写得太低时会自动选用旧版本的tauri,导致和npm端的`@tauri-apps/api`版本不一致,`tauri build`直接报错
3. 开发模式下用`[profile.dev.package.cpu_tasks] opt-level = 3`单独优化计算库;但泛型函数是在调用方crate中编译的,不受这个配置影响,所以`cpu_tasks`对外的计算函数都使用具体类型(如`&mut [u64]`)

### 下一步可以学

1. 更多插件: dialog(对话框)、fs(文件读写)、notification(系统通知)、store(本地存储)
2. 多窗口、系统托盘、菜单
3. 取消长时间运行的任务: 前端点击取消 -> Rust中用`AtomicBool`(rayon)或`CancellationToken`(tokio)通知任务提前结束
