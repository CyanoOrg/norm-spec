# norm-spec

以 Rust library 和可移植 `norm` CLI 实现的 `.norm` 项目约定格式。

本项目从单一、确定性的语义引擎开始。规范、Schema、模板、fixtures 和
机器协议都在本仓库内自足维护；框架适配器消费这些契约，但不成为格式权威。

> 当前状态：`0.1.0-alpha.1`。Gate C 与 Gate D 均已完成。packaged standalone
> adoption 与 framework-neutral canonical Skill 已实现；最终候选 `7e052ae`
> 的 hosted quality、Linux、macOS、Windows CI 全部通过。一次真实的 OpenCode
> 辅助运行也在不启用 pi plugin 的情况下，对 pi-norm-spec 使用了该 Skill。
> Gate E 发布准备实现候选 `3ccde86` 的 hosted quality、MSRV、固定平台及四个
> 原生 artifact jobs 均已通过；`v0.1.0` 正式发布仍是维护者检查点。

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

这是 alpha 开发安装。五个初始命令、compatibility discovery、Rust facade、
任意候选 conformance、packaged standalone adoption lane 与 canonical Skill
目前都已在本地可用。Gate E 增加按目标区分、可自证的候选归档；在维护者批准
公开发布前，CI artifacts 仍只是评审证据。

无插件项目采用、canonical Skill 安装、失败行为与下游 host adapter 边界见
`docs/INTEGRATION.md`。

发行归档、checksum、源码安装、MSRV、升级、回滚与卸载见
`docs/INSTALLATION.md`。

参与开发前请先阅读 `AGENTS.md`、`ROADMAP.md`、`docs/ARCHITECTURE.md` 和
`docs/planning/v0.1-execution.md`。
