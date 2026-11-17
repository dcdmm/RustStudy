# y_tauri

Tauri 2 + React + TypeScript + Vite 入门案例

Tauri: 用Web技术(HTML/CSS/JS)写界面,用Rust写后端逻辑,打包成桌面应用。界面由系统自带的WebView渲染(Windows上是WebView2),不像Electron那样内置一个Chromium,所以安装包很小

参考文档: https://tauri.app

### 运行

```shell
cd y_tauri
npm install                          # 首次运行前安装前端依赖
npm run tauri dev                    # 开发模式,首次需要编译Rust,比较慢
npm run tauri build                  # 打包安装包,产物在src-tauri/target/release/bundle/
npm run tauri build -- --no-bundle   # 只编译exe,不生成安装包,产物在src-tauri/target/release/y_tauri.exe

cd src-tauri && cargo test           # 运行Rust单元测试
```

`npm run tauri dev`做了什么:

1. 执行tauri.conf.json中的`beforeDevCommand`(`npm run dev`),启动Vite开发服务器: http://localhost:1420
2. 用cargo编译并运行src-tauri
3. 打开窗口,加载`devUrl`(就是上面的Vite地址)
4. 修改前端代码 -> 页面热更新;修改Rust代码 -> 自动重新编译并重启应用

`npm run tauri build`做了什么:

1. 执行`beforeBuildCommand`(`npm run build`),把前端打包到dist/
2. release模式编译Rust,并把`frontendDist`(dist/)嵌入到exe中,运行时不再需要Vite
3. 生成安装包(msi/nsis)

### 目录结构

```
y_tauri/
├── index.html                # 前端入口HTML
├── package.json              # 前端依赖和npm脚本
├── vite.config.ts            # Vite配置(固定端口1420,供Tauri加载)
├── src/                      # 前端(React + TypeScript)
│   ├── main.tsx              # React入口
│   ├── App.tsx               # 页面,组合下面4个demo
│   └── demos/                # 每个文件对应一个知识点
└── src-tauri/                # Rust后端,本身就是一个普通的cargo项目
    ├── Cargo.toml
    ├── build.rs              # 编译期处理tauri.conf.json、capabilities等
    ├── tauri.conf.json       # Tauri核心配置: 窗口、前端地址、打包
    ├── capabilities/         # 权限: 前端能调用哪些Tauri API
    ├── icons/                # 应用图标
    └── src/
        ├── main.rs           # 桌面端入口,只调用lib.rs中的run()
        └── lib.rs            # 所有command和应用的构建逻辑
```

### 前端和Rust如何通信

```
前端(WebView)                                  Rust
invoke("greet", { name })  ─── 请求(JSON) ───▶  #[tauri::command] fn greet
                           ◀── 返回值(JSON) ───
listen("countdown", cb)    ◀──── 事件(JSON) ───  app.emit("countdown", payload)
```

1. command: 前端调用Rust,一问一答(类似HTTP请求)
2. 事件: Rust主动推送给前端(类似WebSocket推送)

### 4个示例

建议按顺序学习,每个示例都是Rust端(`src-tauri/src/lib.rs`)和前端(`src/demos/`)对照着看

| 示例 | Rust | 前端 | 知识点 |
| --- | --- | --- | --- |
| 1 | `greet` | `Greet.tsx` | `#[tauri::command]`、`invoke`、参数和返回值 |
| 2 | `divide` | `Divide.tsx` | 返回`Result`,`Err`在前端变为Promise reject |
| 3 | `get_count`、`increment` | `Counter.tsx` | `.manage()`托管状态,`State<'_, T>`自动注入 |
| 4 | `start_countdown` | `Countdown.tsx` | `AppHandle`、`emit`推送事件,`listen`监听事件 |

### 注意点

1. 新写的command必须加到`generate_handler![]`中,否则前端调用报错: Command xxx not found
2. command名在前端保持snake_case;参数名在前端要写成camelCase(`total_seconds` -> `totalSeconds`)
3. 参数和返回值通过JSON传递,所以参数类型要实现`Deserialize`,返回值类型要实现`Serialize`(String、数字、bool、Vec等serde已实现)
4. `State`、`AppHandle`等特殊参数由Tauri自动注入,前端不需要传
5. 普通command在主线程执行,耗时操作要用`async fn`或放到新线程,否则界面会卡住
6. `invoke`只能在Tauri窗口中使用,直接用浏览器打开 http://localhost:1420 会报错
7. Tauri 2的权限系统: `capabilities/default.json`中的`core:default`允许前端使用核心API(如事件`listen`);自己写的command默认都允许调用,不用在这里声明

### 下一步可以学

1. async command: `async fn`,适合文件读写、网络请求等耗时操作
2. 插件: 如官方的dialog、fs、opener插件(`npm run tauri add dialog`),以及在capabilities中配置对应权限
3. 多窗口、系统托盘、菜单
