# fetcher
定时拉取 git 代码的一个桌面应用


# TODO

- [ ] toml config
- [ ] manage cron for git fetch
- [ ] opentelemetry?
- [ ] project module
- [ ] cli

## toml config

```toml
[repos]
    [pulsar]
    url: https://github.com/apache/pulsar.git
    path: /home/lan/git/pulsar
    branche:
        - match:
            expect: xxx
            regx: xxx
            prefix: xxx
          auth:
            username: xx
            password: xx
    time: 1m/1h/30s
```

# 项目模块

1. fetchadmin  用于执行 git fetch 以及管理信息的 的模块
2. fetchcli 用于构建二进制命令行程序,
3. fetchui 桌面应用程序,将会基于 Tauri 框架实现?

初步设计 repo branch那些信息都写在配置文件内，不支持动态处理。

后续可以考虑在 UI 中操作这些信息。