# norm-spec

以 Rust library 和可移植 `norm` CLI 实现的 `.norm` 项目约定格式。

本项目从单一、确定性的语义引擎开始。规范、Schema、模板、fixtures 和
机器协议都在本仓库内自足维护；框架适配器消费这些契约，但不成为格式权威。

> 当前状态：`0.1.0-alpha.1`。Gate C 已集成并通过远端跨平台 CI。
> Gate D1 已在本地完成：高层 Rust facade、CLI 委托、workspace package
> 验证以及精确 Git revision 外部 consumer 均为绿色；本批次仍待远端 CI。

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
```

也可以从当前 checkout 安装开发版本：

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
norm collect --root . --target path/to/directory --pretty
norm validate --all --strict
norm init --profile module --output path/to/.norm
norm scan --root . --text
```

这是 alpha 开发安装。五个冻结子命令与 D1 Rust facade 目前都可用；兼容性
发现、任意候选 conformance、独立采用、Skill 与分发仍属于后续 gates。

参与开发前请先阅读 `AGENTS.md`、`ROADMAP.md`、`docs/ARCHITECTURE.md` 和
`docs/planning/v0.1-execution.md`。
