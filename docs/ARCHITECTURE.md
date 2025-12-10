# 架构设计

## 概述

crater-ohos-bot 是一个基于 Rust 和 axum 框架构建的 Web 服务，用于在代码托管平台上提供 crater-ohos 的用户接口。

## 整体架构

```
┌─────────────────┐
│   GitCode/      │
│   GitHub/Gitee  │
│   (用户界面)     │
└────────┬────────┘
         │ Webhook (Issue 评论)
         ▼
┌─────────────────────────────────────┐
│      crater-ohos-bot                │
│  ┌─────────────────────────────┐   │
│  │   Webhook Receiver          │   │
│  │  - 验证签名                  │   │
│  │  - 解析 Webhook             │   │
│  └──────────┬──────────────────┘   │
│             ▼                       │
│  ┌─────────────────────────────┐   │
│  │   Command Parser            │   │
│  │  - 解析 Bot 指令             │   │
│  └──────────┬──────────────────┘   │
│             ▼                       │
│  ┌─────────────────────────────┐   │
│  │   Command Processor         │   │
│  │  - 处理业务逻辑              │   │
│  │  - 调用 Crater API           │   │
│  └──────────┬──────────────────┘   │
│             ▼                       │
│  ┌─────────────────────────────┐   │
│  │   Platform Adapter          │   │
│  │  - 发布评论                  │   │
│  │  - 管理实验映射              │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │   Callback Handler          │   │
│  │  - 接收 crater 回调          │   │
│  │  - 发布结果通知              │   │
│  └─────────────────────────────┘   │
└────────────┬────────────────────────┘
             │ HTTP API 调用
             ▼
┌─────────────────────────────────────┐
│        crater-ohos 服务             │
│  - 实验管理                          │
│  - 实验执行                          │
│  - 结果生成                          │
└─────────────────────────────────────┘
```

## 核心模块

### 1. Webhook Receiver (`webhook/receiver.rs`)

**职责**:
- 接收来自代码托管平台的 Webhook 事件
- 验证 Webhook 签名确保安全性
- 过滤相关事件（Issue 评论）
- 提取评论内容并传递给命令解析器

**关键流程**:
1. 接收 HTTP POST 请求
2. 验证 `X-GitCode-Token` 头（或其他平台的签名）
3. 检查事件类型（仅处理 `note` 事件）
4. 提取项目、Issue 和评论信息
5. 调用命令解析器

### 2. Command Parser (`bot/commands.rs`)

**职责**:
- 解析评论文本，识别 Bot 指令
- 验证指令格式和参数
- 返回结构化的命令对象

**支持的指令**:
- `run <toolchain1> <toolchain2> [...]` - 运行实验
- `status` - 查询状态
- `abort` - 中止实验
- `list` - 列出实验
- `help` - 显示帮助

**解析逻辑**:
1. 检查评论是否以触发前缀开头（默认 `@crater-bot`）
2. 提取指令名称和参数
3. 验证参数数量和格式
4. 返回枚举类型的命令对象

### 3. Command Processor (`bot/processor.rs`)

**职责**:
- 处理解析后的命令
- 调用 crater-ohos API
- 生成响应消息
- 管理实验与 Issue 的关联

**关键方法**:
- `handle_run()` - 创建并启动实验
- `handle_status()` - 查询实验状态
- `handle_abort()` - 中止运行中的实验
- `handle_list()` - 列出所有实验
- `handle_help()` - 生成帮助文本

**实验命名规则**:
- 格式: `{project}-{issue_id}`
- 示例: `username-repo-123`
- 用于关联 Issue 和实验

### 4. Crater Client (`crater/client.rs`)

**职责**:
- 封装 crater-ohos REST API
- 处理认证（Bearer Token）
- 错误处理和重试
- 类型安全的请求/响应

**API 方法**:
- `create_experiment()` - 创建实验
- `list_experiments()` - 列出实验
- `get_experiment()` - 获取实验详情
- `run_experiment()` - 运行实验
- `abort_experiment()` - 中止实验
- `delete_experiment()` - 删除实验

### 5. Platform Adapter (`platforms/`)

**职责**:
- 抽象不同平台的 API 差异
- 统一的评论发布接口
- 实验映射存储（Issue ↔ Experiment）
- Webhook 验证

**Trait 定义**:
```rust
pub trait PlatformAdapter {
    async fn post_comment(&self, project: &str, issue_id: u64, content: &str);
    fn verify_webhook(&self, payload: &[u8], signature: &str) -> bool;
    async fn store_experiment_mapping(&self, ...);
    async fn get_experiment_mapping(&self, ...) -> Option<String>;
}
```

**实现**:
- `GitCodeAdapter` - 完整实现
- `GitHubAdapter` - 接口预留
- `GiteeAdapter` - 接口预留

### 6. Callback Handler (`webhook/callback.rs`)

**职责**:
- 接收 crater-ohos 发送的实验状态回调
- 解析实验名称提取 Issue 信息
- 根据状态生成通知消息
- 在对应 Issue 中发布结果

**回调处理流程**:
1. 接收 crater-ohos 的 POST 请求
2. 解析实验名称（`{project}-{issue_id}`）
3. 根据状态生成消息（completed/failed/aborted）
4. 调用 Platform Adapter 发布评论

## 数据流

### 用户发起实验

```
用户 → GitCode Issue 评论 "@crater-bot run stable beta"
  ↓
GitCode → Webhook → Bot
  ↓
Bot: 解析指令
  ↓
Bot: 创建实验 → crater-ohos API
  ↓
Bot: 发布确认评论 → GitCode API
  ↓
用户看到: "✅ 实验已创建..."
```

### 实验完成通知

```
crater-ohos: 实验完成
  ↓
crater-ohos → 回调 → Bot
  ↓
Bot: 解析回调
  ↓
Bot: 发布结果评论 → GitCode API
  ↓
用户看到: "🎉 实验已完成！📊 查看报告..."
```

## 技术选型

### Web 框架: axum
- **优势**: 高性能、类型安全、基于 tokio
- **用途**: HTTP 服务器、路由、中间件

### HTTP 客户端: reqwest
- **优势**: 功能完整、异步支持、易用
- **用途**: 调用 crater-ohos 和平台 API

### 异步运行时: tokio
- **优势**: 成熟、高性能、生态丰富
- **用途**: 异步任务调度

### 配置管理: config + toml
- **优势**: 灵活、类型安全、易读
- **用途**: 加载和验证配置

### 日志: tracing
- **优势**: 结构化日志、性能优秀
- **用途**: 调试和监控

## 安全考虑

1. **Webhook 验证**: 所有 Webhook 必须通过签名验证
2. **API 认证**: crater-ohos API 使用 Bearer Token
3. **输入验证**: 所有用户输入都经过严格验证
4. **错误处理**: 不暴露敏感信息到错误消息

## 扩展性设计

### 添加新平台支持

1. 在 `platforms/` 下创建新文件
2. 实现 `PlatformAdapter` trait
3. 在 `config.rs` 添加平台配置
4. 在 `main.rs` 初始化适配器

### 添加新指令

1. 在 `BotCommand` 枚举添加变体
2. 在 `BotCommand::parse()` 添加解析逻辑
3. 在 `CommandProcessor` 添加处理方法

## 性能考虑

- **异步 I/O**: 所有网络操作都是异步的
- **并发处理**: 多个 Webhook 可以同时处理
- **资源限制**: 合理的超时和重试策略

## 未来改进

1. **持久化存储**: 使用数据库替代内存存储实验映射
2. **队列系统**: 使用消息队列处理大量 Webhook
3. **监控和指标**: 添加 Prometheus 指标
4. **缓存**: 缓存频繁查询的数据
5. **更多平台**: 完整实现 GitHub 和 Gitee 支持
