# norm-spec

以 Rust library 和可移植 `norm` CLI 实现的 `.norm` 项目约定格式。

本项目从单一、确定性的语义引擎开始。规范、Schema、模板、fixtures 和
机器协议都在本仓库内自足维护；框架适配器消费这些契约，但不成为格式权威。

> 当前状态：`0.1.0-alpha.1`。Gate C 的第一个可执行纵向切片已在本地完成：
> 全局 help/version 与 `norm parse` 已满足冻结的 17 个 global/parse cases。
> collect、validate、init 与 scan 尚未实现。

## 开发版本使用

直接从工作区运行：

```bash
cargo run -p norm-spec-cli -- --version
cargo run -p norm-spec-cli -- parse path/to/.norm --pretty
```

也可以从当前 checkout 安装开发版本：

```bash
cargo install --path crates/norm-cli --locked
norm parse path/to/.norm --pretty
```

这是 alpha 开发安装。目前只有 `parse` 可用；其余四个已冻结的子命令会
明确返回退出码 `2`，直到各自的 Gate C 切片完成。

参与开发前请先阅读 `AGENTS.md`、`ROADMAP.md`、`docs/ARCHITECTURE.md` 和
`docs/planning/v0.1-execution.md`。
