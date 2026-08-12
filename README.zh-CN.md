# norm-spec

以 Rust library 和可移植 `norm` CLI 实现的 `.norm` 项目约定格式。

本项目从单一、确定性的语义引擎开始。规范、Schema、模板、fixtures 和
机器协议都在本仓库内自足维护；框架适配器消费这些契约，但不成为格式权威。

> 当前状态：`0.1.0-rc.1` 发布准备候选。Gate C、Gate D 与 Gate E 分发实现均
> 已完成，包括 standalone 与 canonical Skill adoption、Rust 1.97 MSRV 验证
> 和四个原生归档目标。RC 仍需对精确候选做 hosted 验证；公开可见性、tag、
> GitHub Pre-release、crates.io 发布以及稳定版 `v0.1.0` 都仍是维护者检查点。

## Rust API

在 registry 发布前，应固定到精确 Git revision：

```toml
[dependencies]
norm-spec = { git = "https://github.com/CyanoOrg/norm-spec", rev = "<exact-commit>" }
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

## 开发版本使用

直接从工作区运行：

```bash
cargo run -p norm-spec-cli -- --version
cargo run -p norm-spec-cli -- parse path/to/.norm --pretty
cargo run -p norm-spec-cli -- collect --root . --target path/to/directory --pretty
cargo run -p norm-spec-cli -- validate --all --strict
cargo run -p norm-spec-cli -- init --profile module --output path/to/.norm
cargo run -p norm-spec-cli -- scan --root . --text
cargo run -p norm-spec-cli -- compatibility --pretty
```

也可以从当前 checkout 安装开发版本：

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
norm collect --root . --target path/to/directory --pretty
norm validate --all --strict
norm init --profile module --output path/to/.norm
norm scan --root . --text
norm compatibility --pretty
```

`cargo install` 也会安装 `norm-spec-conformance`。它使用精确导出或随发布
提供的 contract bundle 验证显式候选：

```bash
norm-spec-conformance \
  --candidate "$(command -v norm)" \
  --contract-dir path/to/exact-contract-bundle \
  --pretty
```

这是尚未公开发布的 RC 候选安装。五个初始命令、compatibility discovery、
Rust facade、任意候选 conformance、packaged standalone adoption lane、
canonical Skill 以及按目标区分的可自证归档目前都已可用。在维护者批准公开
发布动作前，CI artifacts 仍只是评审证据。

无插件项目采用、canonical Skill 安装、失败行为与下游 host adapter 边界见
`docs/INTEGRATION.md`。

发行归档、checksum、源码安装、MSRV、升级、回滚与卸载见
`docs/INSTALLATION.md`。

参与开发前请先阅读 `AGENTS.md`、`ROADMAP.md`、`docs/ARCHITECTURE.md` 和
`docs/planning/v0.1-execution.md`。
