# norm-spec

以 Rust library 和可移植 `norm` CLI 实现的 `.norm` 项目约定格式。

本项目从单一、确定性的语义引擎开始。规范、Schema、模板、fixtures 和
机器协议都在本仓库内自足维护；框架适配器消费这些契约，但不成为格式权威。

> 当前状态：`0.1.0-rc.1` 已从精确提交 `5c781964` 发布并完成验证。签名 tag、
> 四个原生归档及 checksum、三个 Rust package、docs.rs 页面、standalone
> adoption 和 canonical Skill 路径均已公开。稳定版 `v0.1.0` 仍需在 RC soak
> 后由维护者单独决策。

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

这个公开 RC 包含五个初始命令、compatibility discovery、Rust facade、任意
候选 conformance、packaged standalone adoption、canonical Skill 以及按目标
区分的可自证归档；它还不是稳定版 `v0.1.0`。

无插件项目采用、canonical Skill 安装、失败行为与下游 host adapter 边界见
`docs/INTEGRATION.md`。

发行归档、checksum、源码安装、MSRV、升级、回滚与卸载见
`docs/INSTALLATION.md`。

参与开发前请先阅读 `AGENTS.md`、`ROADMAP.md`、`docs/ARCHITECTURE.md` 和
`docs/planning/v0.1-execution.md`。
