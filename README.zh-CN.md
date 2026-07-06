# Presence Guard

> **面向桌面设备的本地化、Rust-first 在场保护能力。**
>
> 当已录入用户不在屏幕前、出现陌生观看者或多人围观时，系统根据统一策略自动保护屏幕内容，并由平台适配层请求系统锁屏。

[English](README.md) · [架构说明](docs/architecture.md) · [贡献指南](CONTRIBUTING.md) · [安全说明](SECURITY.md) · [发布流程](RELEASING.md)

## 为什么需要 Presence Guard

传统的系统空闲锁屏只能判断“长时间没有输入”，无法区分：

- 已授权用户仍在屏幕前；
- 用户已经离开；
- 陌生人正在观看屏幕；
- 多人同时出现在镜头中；
- 摄像头画面模糊、被遮挡或不可用。

Presence Guard 提供本地、可复用的保护决策层：

- **已授权用户在场**：保持正常使用；
- **持续无人脸**：达到配置时长后请求系统锁屏；
- **陌生人连续出现**：先显示隐私遮罩，再请求系统锁屏；
- **多人出现**：优先显示隐私遮罩，不把多人场景等同于身份识别失败；
- **信号不可靠**：进入不可用状态，而不是直接误锁。

项目以本地处理为原则：不调用云端身份服务，不上传摄像头帧。

## 项目状态

| 组件 | 状态 | 范围 |
| --- | --- | --- |
| `presence-core` | 已实现 | 纯 Rust 策略状态机与单元测试。 |
| macOS 适配层 | 正在进行导入校验 | AVFoundation/Vision/AppKit 桥接、菜单栏、隐私遮罩与系统锁屏执行器。 |
| Windows 适配层 | 规划中 | 仅替换平台集成，策略保持复用。 |
| HarmonyOS PC 适配层 | 规划中 | 仅替换平台集成，策略保持复用。 |

macOS 源码不会未经验证直接进入标准源码树：GitHub Actions 会先校验暂存压缩包的 SHA-256、限制可解压路径、在 macOS runner 编译，全部通过后再提交源码并清理临时分片。

## 架构

```text
摄像头 / 人脸引擎 ──> FaceSignal ┐
                                  ├── presence-core（Rust）──> ProtectionAction ──> 平台执行器
系统锁屏 / 隐私遮罩 <─────────────┘
```

`presence-core` 负责状态机、防抖、多帧确认、异常信号处理和保护决策。每个平台只需要提供两类适配能力：

1. 产生标准化身份/在场信号；
2. 执行隐私遮罩和系统会话锁屏。

因此 Windows、macOS、HarmonyOS PC 可以复用同一套策略语义，而不必强行共享各自不同的摄像头或系统 API。

## 仓库结构

```text
.
├── crates/
│   └── presence-core/          # 跨端 Rust 策略核心
├── docs/                       # 架构与项目文档
├── .github/workflows/          # CI、Release、受控源码导入流程
├── CONTRIBUTING.md
├── SECURITY.md
├── RELEASING.md
└── README.md
```

## 快速开始

### 环境要求

- Rust stable
- Git

### 校验 Rust Core

```bash
cargo fmt --all -- --check
cargo clippy -p presence-core --all-targets -- -D warnings
cargo test -p presence-core --all-targets
```

macOS App 还需要 macOS、Xcode Command Line Tools、可用摄像头以及平台适配层源码；它不是一个可在三端直接运行的通用二进制 crate。

## 隐私与安全边界

- 项目面向本地在场与人脸比对信号处理。
- 它不能替代系统登录、FileVault、企业终端管理或经认证的活体检测能力。
- 摄像头不确定或不可用，不得被视为“出现陌生人”的证据。
- 若产品版本持久化生物特征材料，必须先完成必要性评估、用户同意、加密保护与适用的隐私合规要求。

漏洞报告与安全边界见 [SECURITY.md](SECURITY.md)。

## CI 与发布

- **CI**：在 Linux、macOS、Windows runner 上执行格式检查、Clippy、测试和 Core 构建。
- **Release**：推送符合 `vMAJOR.MINOR.PATCH` 的标签后，校验 Rust Core，并发布 `presence-core` 源码包及 `SHA256SUMS.txt`。
- **macOS 源码导入**：用于处理无法经远程写入接口直接提交的源码，属于受校验的一次性导入流程。

发布命令和发布门槛见 [RELEASING.md](RELEASING.md)。

## 贡献

欢迎贡献。请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md)，保持平台代码位于适配层边界内，并且不要提交人脸样本、令牌、构建产物或本地审计日志。

## 许可证

MIT，详见 [LICENSE](LICENSE)。
