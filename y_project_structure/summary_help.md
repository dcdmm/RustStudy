```text
├── src/
│ ├── lib.rs
│ ├── main.rs
│ ├── util.rs
│ └── util
| ├──config.rs
```

| Module Path         | Filesystem Path    | File Contents |
|---------------------|--------------------|---------------|
| crate               | src/lib.rs         | mod util;     |
| crate               | src/main.rs        | mod util;     |
| crate::util         | src/util.rs        | mod config;   |
| crate::util::config | src/util/config.rs |               |