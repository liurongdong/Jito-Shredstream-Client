# Jito Shredstream Client

这是一个用于与 Jito 网络交互的 Rust 客户端实现。该客户端主要用于处理 Solana 区块链上的 shredstream 数据。

## 项目概述

Jito Shredstream Client 是一个专门设计用来与 Jito 协议交互的客户端库，它基于 Solana 区块链技术构建，提供了高效的数据流处理能力。

## 技术栈

- Rust 2024 edition
- Solana SDK v2.2.1
- Jito 协议集成
- Tokio 异步运行时

## 主要依赖

- `jito-protos`: Jito 协议的 protobuf 定义
- `solana-sdk`: Solana 的核心 SDK
- `solana-client`: Solana 客户端库
- `solana-metrics`: 性能指标监控
- `solana-perf`: 性能优化组件
- `solana-streamer`: 数据流处理
- `tokio`: 异步运行时支持
- `bincode`: 二进制序列化
- `serde_json`: JSON 序列化支持

## 构建要求

- Rust 2024 edition
- Cargo 包管理器

## 如何使用

1. 克隆仓库：
```bash
git clone https://github.com/vnxfsc/Jito_Shredstream_Client.git
cd jito-shredstream-client
```

2. 构建项目：
```bash
cargo build
```

3. 运行测试：
```bash
cargo test
```

## 许可证

待定

## 贡献指南

欢迎提交 Pull Requests 和 Issues。请确保在提交代码前：
1. 代码符合 Rust 标准格式规范
2. 所有测试通过
3. 添加适当的文档注释

## 注意事项

- 本项目仍在开发中
- 使用了特定版本的 Solana 依赖 (v2.2.1)
- 部分功能依赖于 Jito Foundation 的修改版 Solana 