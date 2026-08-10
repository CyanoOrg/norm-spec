# norm-spec

以 Rust library 和可移植 `norm` CLI 实现的 `.norm` 项目约定格式。

本项目从单一、确定性的语义引擎开始。规范、Schema、模板、fixtures 和
机器协议都在本仓库内自足维护；框架适配器消费这些契约，但不成为格式权威。

> 当前状态：`0.1.0-alpha.1` 启动阶段。治理和契约骨架已经建立，parser、
> validator 与正式 CLI 行为尚未完成。

参与开发前请先阅读 `AGENTS.md`、`ROADMAP.md`、`docs/ARCHITECTURE.md` 和
`docs/planning/v0.1-execution.md`。
