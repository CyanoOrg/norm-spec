# norm-spec

`.norm` 项目约定格式，以及唯一拥有其语义的确定性 Rust 引擎。

## 为什么

每个项目都有一套从未被完整写下的规则——命名习惯、模块边界、提交粒度、
"我们这里不这么干"。它们活在 review 者的脑子里、旧决策和口口相传的经验中。
新人如此，agent 更是如此：不知道这些规则的代价是返工、review 摩擦和悄悄的
divergence。

常见的答案——一份常驻的指令文件——把约定知识当作记忆：一次装入，期望一直
有效。但 LLM 上下文的行为像缓存，不像记忆。指令的效力随距离和竞争 token
衰减；长会话会忘记开头说过什么，而重读一份平铺文件只是花同样的 token
重建同样的歧义。

norm-spec 从一个不同的命题出发：

> 项目约定应当以带作用域、可校验的形态常驻磁盘——与代码同级，而不是一段散文。

`.norm` 文件是分层的（目录继承）、带作用域的（每条约定声明自己的目标）、
可校验的（schema、引用完整性、单一事实源）。解析、收集与校验语义由本仓库
唯一的确定性引擎拥有，连同规范、schema、fixtures 和机器协议一起自足维护。
宿主适配器（[pi-norm-spec](https://github.com/CyanoOrg/pi-norm-spec)、
[dsh-norm-spec](https://github.com/CyanoOrg/dsh-norm-spec)）消费这些契约，
在行动时刻把精确收集的约定递送进 agent 会话、并在编辑后核对——但不成为
格式权威。结果是一个闭环而非散文：写作时校验、收集时定界、递送时可见。

## Crates

- `norm-spec`：打包的、文件系统感知的 collect 与校验 facade。
- `norm-spec-core`：确定性解析、收集规则、Schema 与语义校验、版本化响应模型。
- `norm-spec-cli`：参数、展示、退出码、`norm` 命令与独立的
  `norm-spec-conformance` runner；文件系统语义与 release 资产委托给 `norm-spec`。

## Rust API

使用经过评审的精确 RC 版本：

```toml
[dependencies]
norm-spec = "=0.1.0-rc.1"
```

```rust
use std::path::Path;
use norm_spec::{CollectRequest, ValidateRequest, collect, validate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(".");
    let inherited = collect(CollectRequest::new(root, Path::new("docs")))?;
    let checked = validate(ValidateRequest::all(root))?;
    assert!(!inherited.norms.is_empty());
    assert_eq!(checked.summary.errors, 0);
    Ok(())
}
```

请求、失败分类、兼容性与打包边界见 `docs/RUST-API.md`。

## 安装发布候选

从 crates.io 安装两个 CLI executable：

```bash
cargo install norm-spec-cli --version '=0.1.0-rc.1' --locked
norm --version
norm compatibility --pretty
```

GitHub Pre-release 提供各目标平台归档及对应 checksum。将 binary 复制进
`PATH` 前，请按 `docs/INSTALLATION.md` 完成验证。

## 开发用法

直接从 checkout 运行：

```bash
cargo run -p norm-spec-cli -- --version
cargo run -p norm-spec-cli -- parse path/to/.norm --pretty
cargo run -p norm-spec-cli -- collect --root . --target path/to/directory --pretty
cargo run -p norm-spec-cli -- validate --all --strict
cargo run -p norm-spec-cli -- init --profile module --output path/to/.norm
cargo run -p norm-spec-cli -- scan --root . --text
cargo run -p norm-spec-cli -- compatibility --pretty
```

或从本 checkout 安装当前开发版 binary：

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
norm collect --root . --target path/to/directory --pretty
norm validate --all --strict
norm init --profile module --output path/to/.norm
norm scan --root . --text
norm compatibility --pretty
```

`cargo install` 同时安装 `norm-spec-conformance`。它对显式候选与精确导出
或 release 提供的契约 bundle 做核对：

```bash
norm-spec-conformance \
  --candidate "$(command -v norm)" \
  --contract-dir path/to/exact-contract-bundle \
  --pretty
```

## 状态

`0.1.0-rc.1` 已发布并完成验证：签名 tag `v0.1.0-rc.1`、三个 crate 上线
crates.io、docs.rs 页面、四个带 checksum 的原生归档、canonical Skill 路径。
稳定版 `v0.1.0` 的提升遵循 `ROADMAP.md` 记录的 RC soak 判据——两个独立
下游适配器在已发布版本中消费 `0.1.0-rc.1`，加上第二个发布后的 soak 窗口。

## 文档

- `docs/INTEGRATION.md`——无插件的项目接入、canonical Skill 安装、失败
  行为与下游宿主适配器边界
- `docs/INSTALLATION.md`——release 归档、checksum、源码安装、MSRV、
  升级、回滚与卸载
- `docs/ARCHITECTURE.md`——crate 边界与各行为归属
- `docs/RUST-API.md`——Rust facade 的请求、失败与兼容性
- `ROADMAP.md`、`docs/planning/v0.1-execution.md`——里程碑与执行状态，
  贡献前请先阅读

## 许可证

MIT © 2026 Wade
