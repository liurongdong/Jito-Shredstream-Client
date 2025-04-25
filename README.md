# Jito Shredstream 客户端

这是一个基于 Rust 的客户端应用程序，用于连接 Jito Shredstream 服务，监听 Solana 区块链上的交易并进行实时解析和分析。

## 功能特点

- 连接 Jito Shredstream 服务获取 Solana 实时交易数据
- 根据配置监控特定账户的相关交易
- 支持解析 Pump 协议交易
- 支持解析 Pump AMM 协议交易
- 打印详细的交易信息，包括签名、账户、代币和指令信息

## 交易解析功能

### Pump 协议交易解析
支持解析以下 Pump 交易类型：
- 获取曲线信息 (Get Bonding Curve Info)
- 识别代币 Mint 地址
- 识别曲线账户 (Curve Account)
- 曲线状态跟踪 (完成/进行中)
- 特殊账户识别 (带有 "pump" 后缀的账户)

### Pump AMM 协议交易解析
支持解析以下 Pump AMM 交易操作：
- CreatePool (创建流动性池) - 解析池索引、基础代币输入量、报价代币输入量
- Deposit (存入流动性) - 解析LP代币输出量、最大基础代币输入量、最大报价代币输入量
- Buy (买入代币) - 解析基础代币输出量、最大报价代币输入量
- Sell (卖出代币) - 解析基础代币输入量、最小报价代币输出量
- Withdraw (提取流动性) - 解析LP代币输入量、最小基础代币输出量、最小报价代币输出量

此外，还能提取以下信息：
- 池地址 (Pool Address)
- 基础代币和报价代币的 Mint 地址
- LP 代币 Mint 地址

## 环境要求

- Rust 1.65+
- Solana SDK 1.17.0+
- Jito Protos 依赖
## 服务端安装教程
```bash
git clone https://github.com/jito-labs/shredstream-proxy.git
cd shredstream-proxy
#启动
RUST_LOG=info cargo run --release --bin jito-shredstream-proxy -- shredstream \
    --block-engine-url https://mainnet.block-engine.jito.wtf \ //
    --auth-keypair keypair.json \  //jito_shred 私钥
    --desired-regions amsterdam,ny \  //要接受的区域
    --dest-ip-ports 127.0.0.1:8001,10.0.0.1:8001
    --grpc-service-port 9999
```

## Clinet安装

```bash
git clone https://github.com/vnxfsc/jito-shredstream-client.git
cd jito-shredstream-client
cargo build --release
```

## 配置

系统默认通过环境变量进行配置，主要配置项包括：

- `SHREDSTREAM_SERVER_URL` - Jito Shredstream 服务器地址（默认为 "http://127.0.0.1:9999"）
- `CREATE_ACCOUNT` - 创建代币交易相关的目标账户（默认为 "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"）
- `SWAP_ACCOUNT` - Swap 交易相关的目标账户（默认为 "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"）

项目还内置了对特定协议账户的监控：
- Pump AMM 程序地址：`pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`

## 运行

```bash
cargo run --release
```

运行后，程序将连接到 Jito Shredstream 服务器并开始监听配置中指定的账户相关交易。当监测到符合条件的交易时，会打印出详细的交易信息。

## 项目结构

```
jito-shredstream-client/
├── src/
│   ├── main.rs              # 主程序入口，包含主循环和交易处理逻辑
│   ├── config/              # 配置模块
│   │   └── mod.rs           # 配置实现，处理环境变量和默认配置
│   ├── client/              # Jito Shredstream客户端
│   │   └── mod.rs           # 连接和订阅逻辑实现
│   ├── transaction/         # 交易解析模块
│   │   ├── mod.rs           # 通用交易处理函数，包括交易信息打印和分组
│   │   ├── pump_parser.rs   # Pump协议交易解析实现
│   │   ├── pumpamm_parser.rs # Pump AMM协议交易解析实现
│   │   └── IDL/             # 接口定义文件
│   │       ├── pump_idl.json    # Pump协议IDL
│   │       └── pumpamm_idl.json # Pump AMM协议IDL
│   └── jito_protos/         # Jito协议相关定义
│       ├── build.rs         # 构建脚本，用于生成协议代码
│       ├── Cargo.toml       # Jito Protos子包配置
│       ├── protos/          # 协议定义文件
│       └── src/             # 生成的协议代码
├── Cargo.toml               # 项目依赖配置
└── Cargo.lock               # 锁定的依赖版本
```

### 主要模块功能

#### main.rs
- 程序入口点，设置配置并启动客户端
- 实现主循环，处理连接重试逻辑
- 处理接收到的交易数据，分组并打印交易信息

#### config
- 管理应用程序配置
- 处理环境变量读取和默认值设置
- 定义常量如程序ID等

#### client
- 实现与Jito Shredstream服务的连接
- 处理订阅请求和响应流
- 管理重连逻辑

#### transaction
- **mod.rs**: 通用交易处理逻辑，打印交易细节，按账户分组交易
- **pump_parser.rs**: 解析Pump协议交易，包括代币创建和曲线相关操作
- **pumpamm_parser.rs**: 解析PumpAMM协议交易，支持流动性池操作
- **IDL/**: 包含Solana程序接口定义，用于正确解析交易指令

#### jito_protos
- 包含Jito服务协议定义和生成的代码
- 实现与Jito服务的通信协议

## 示例输出

### Pump AMM CreatePool 交易示例
```
开始监听目标账户的交易...
监控账户: TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM
监控账户: Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1
监控账户: pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA

找到账户 pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA 的 1 笔新交易 当前Slot:[12345678]
===== Pump AMM协议交易 =====

交易 1:

交易详情:
签名: 5Kd7EuJP4GK8tVNHmjPgxUTKPkgifJKJVAMRKVRCgN7SqQ4Ri7CpAr49rsMkUFqoZcieawt23QPmPhw9VwW2FXMF
消息版本: Legacy(LegacyMessage { header: MessageHeader { num_required_signatures: 1, num_readonly_signed_accounts: 0, num_readonly_unsigned_accounts: 7 }, account_keys: [C4LQDpMVnxZRqyR8xArhHxKL1Ls5tYZKBUkZrMJnXgbX, pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA, ...], ...})

Pump特殊账户:
  不存在

签名账户: C4LQDpMVnxZRqyR8xArhHxKL1Ls5tYZKBUkZrMJnXgbX

交易类型: 单签名交易

指令详情:
  指令 0:
    程序: ComputeBudget111111111111111111111111111111
    类型: 计算预算指令
    操作: 设置计算单元限制
  指令 1:
    程序: ComputeBudget111111111111111111111111111111
    类型: 计算预算指令
    操作: 设置优先级费用
  指令 2:
    程序: pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
    类型: Pump AMM协议指令
    操作: CreatePool
    内容: 创建池: 索引=42, 基础代币输入=100000000, 报价代币输入=1000000000
    池地址: G1uXm5d3KNXGhLsVC9Nw5hNXLt5kDQ2TFvzGjwQuKEKP
    基础代币: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    报价代币: So11111111111111111111111111111111111111112
    相关账户:
      - C4LQDpMVnxZRqyR8xArhHxKL1Ls5tYZKBUkZrMJnXgbX
      - G1uXm5d3KNXGhLsVC9Nw5hNXLt5kDQ2TFvzGjwQuKEKP
      - EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
      - So11111111111111111111111111111111111111112
      - G7G8ErkPXP4ib9bxqpJAyBbRseQQTQRqym4jgBJnR3nE
      - 11111111111111111111111111111111
      - TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Pump AMM协议交易解析:
  Pump AMM指令 1:
    类型: CreatePool
    详情: 创建池: 索引=42, 基础代币输入=100000000, 报价代币输入=1000000000

池信息:
  池地址: G1uXm5d3KNXGhLsVC9Nw5hNXLt5kDQ2TFvzGjwQuKEKP
  基础代币: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
  报价代币: So11111111111111111111111111111111111111112
  LP代币: G7G8ErkPXP4ib9bxqpJAyBbRseQQTQRqym4jgBJnR3nE

操作类型: 创建流动性池

----------------------------------------------
```

### Pump AMM Buy 交易示例
```
找到账户 pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA 的 1 笔新交易 当前Slot:[12345987]
===== Pump AMM协议交易 =====

交易 1:

交易详情:
签名: 2FJLNCUhLSVY4RmAyBDKfeN7xGzP3nqAGP1UsMqonEvZiVBNpg284Gd2BQh9XQ1UZKm2s9bkzJZYkQBdZFXETJ4L
消息版本: Legacy(LegacyMessage { header: MessageHeader { num_required_signatures: 1, num_readonly_signed_accounts: 0, num_readonly_unsigned_accounts: 6 }, account_keys: [...] })

账户数量: 12
指令数量: 3
交易类型: 单签名交易

指令详情:
  指令 0:
    程序: ComputeBudget111111111111111111111111111111
    类型: 计算预算指令
    操作: 设置计算单元限制
  指令 1:
    程序: ComputeBudget111111111111111111111111111111
    类型: 计算预算指令
    操作: 设置优先级费用
  指令 2:
    程序: pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
    类型: Pump AMM协议指令
    操作: Buy
    内容: 买入: 基础代币输出=5000000, 最大报价代币输入=52631579
    池地址: G1uXm5d3KNXGhLsVC9Nw5hNXLt5kDQ2TFvzGjwQuKEKP
    基础代币: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    报价代币: So11111111111111111111111111111111111111112
    相关账户:
      - 5DKQvnDfW4kLfKiwwXTvs8Qjj8yNPkPZpgCzWYZLpUYc
      - G1uXm5d3KNXGhLsVC9Nw5hNXLt5kDQ2TFvzGjwQuKEKP
      - 9vpsmXhZQpFX8LV8zgZXFRrpV6WMvjxZMJ6MMwGxsoXm
      - 7znj4iApBxNBzCX3rQHYQJQmcC3TPiWDFWQbcwZUYE9Y
      - EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
      - So11111111111111111111111111111111111111112
      - 11111111111111111111111111111111
      - TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Pump AMM协议交易解析:
  Pump AMM指令 1:
    类型: Buy
    详情: 买入: 基础代币输出=5000000, 最大报价代币输入=52631579

操作类型: 买入代币

----------------------------------------------
```

### Pump 协议交易示例
```
找到账户 TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM 的 1 笔新交易 当前Slot:[12346789]

交易 1:

交易详情:
签名: 3rJ3ZJQCFySNF9mv6X1rAiMj9Swe2G7k32eA7y44HhrGLC1R8NeKBrUK2ZAUWuWD4WKcYNgFPB84PJikfTBUTZTS
消息版本: Legacy(LegacyMessage { header: MessageHeader { num_required_signatures: 1, num_readonly_signed_accounts: 0, num_readonly_unsigned_accounts: 4 }, account_keys: [...] })

账户数量: 8
指令数量: 2
交易类型: 单签名交易

识别的代币Mint: 8XSsNvaKU9FDhYWAv9Qi5g1NQbrtcbGPJ7RNTG8C6SLL
识别的曲线账户: J3kQRkhHSwKMsEHWQPBYKaYZYpV8ZeMxS2e5pBaCcKPG
曲线状态: 进行中

指令详情:
  指令 0:
    程序: ComputeBudget111111111111111111111111111111
    类型: 计算预算指令
    操作: 设置计算单元限制
  指令 1:
    程序: TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM
    类型: 其他程序指令
    相关账户:
      - 4QdGYZTiVHLwKQBJ4fZNsE6efs2XY6z4HqvT4SMfex2L
      - J3kQRkhHSwKMsEHWQPBYKaYZYpV8ZeMxS2e5pBaCcKPG
      - 3PVyU3hVZPYQPCvm7HtPFq9xdLj411rVM5h3KJ7iSXmK
      - 8XSsNvaKU9FDhYWAv9Qi5g1NQbrtcbGPJ7RNTG8C6SLL
      - 11111111111111111111111111111111

Pump协议交易解析:
  Pump指令 1:
    类型: TransferPDA
    详情: 转移PDA拥有权: 新拥有者=3PVyU3hVZPYQPCvm7HtPFq9xdLj411rVM5h3KJ7iSXmK

曲线信息:
  代币Mint: 8XSsNvaKU9FDhYWAv9Qi5g1NQbrtcbGPJ7RNTG8C6SLL
  曲线账户: J3kQRkhHSwKMsEHWQPBYKaYZYpV8ZeMxS2e5pBaCcKPG
  状态: 进行中

----------------------------------------------
```

## 联系方式

- 交流群：[Buff社区](https://t.me/chainbuff)

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

MIT License
