# crater-ohos-bot

一个用于与 [crater-ohos](https://github.com/LuuuXXX/crater-ohos) 服务交互的 Bot，支持在代码托管平台（优先支持 GitCode）上接收用户指令并执行 crater 实验。

## 功能特性

- 🤖 **多平台支持**: 支持 GitCode（已实现）、GitHub 和 Gitee（接口预留）
- 🔧 **完整的 crater-ohos API 集成**: 支持创建、运行、查询和中止实验
- 📝 **简单的指令系统**: 通过评论发送指令来控制实验
- 🔔 **实时回调通知**: 实验完成后自动在 Issue 中发布结果
- 🛡️ **安全的 Webhook 验证**: 支持 Webhook 签名验证
- ⚙️ **灵活的配置**: 基于 TOML 的配置文件

## 快速开始

### 前置要求

- Rust 1.70+
- 运行中的 crater-ohos 服务实例
- GitCode 账户和访问令牌

### 安装

1. 克隆仓库:
```bash
git clone https://github.com/LuuuXXX/crater-ohos-bot.git
cd crater-ohos-bot
```

2. 复制并编辑配置文件:
```bash
cp config.example.toml config.toml
# 编辑 config.toml 填入你的配置
```

3. 构建项目:
```bash
cargo build --release
```

4. 运行 Bot:
```bash
./target/release/crater-ohos-bot
```

### Docker 部署

```bash
# 构建镜像
docker build -t crater-ohos-bot .

# 运行容器
docker run -d \
  -p 8080:8080 \
  -v $(pwd)/config.toml:/app/config.toml \
  --name crater-ohos-bot \
  crater-ohos-bot
```

## 配置说明

详细的配置说明请参见 `config.example.toml`。主要配置项包括：

- **服务器设置**: 监听地址和端口
- **Crater 服务**: API URL、认证令牌和回调地址
- **平台配置**: GitCode/GitHub/Gitee 的 API 凭据和 Webhook 密钥
- **Bot 设置**: Bot 名称、触发前缀和默认参数

## 支持的指令

在 GitCode Issue 中使用以下指令：

- `@crater-bot run <toolchain1> <toolchain2>` - 创建并运行实验
- `@crater-bot status` - 查看当前实验状态
- `@crater-bot abort` - 中止当前实验
- `@crater-bot list` - 列出所有实验
- `@crater-bot help` - 显示帮助信息

### 示例

```
@crater-bot run stable beta
@crater-bot run nightly-2024-01-01 stable
@crater-bot status
```

详细的指令说明请参见 [docs/COMMANDS.md](docs/COMMANDS.md)。

## 与 crater-ohos 的关系

crater-ohos-bot 是 crater-ohos 的配套组件：

- **crater-ohos**: 提供核心的实验执行引擎和 REST API
- **crater-ohos-bot**: 作为用户界面，在代码托管平台上接收指令并调用 crater-ohos API

两者通过 HTTP REST API 通信，可以独立部署和扩展。

## 文档

- [架构设计](docs/ARCHITECTURE.md)
- [部署指南](docs/DEPLOYMENT.md)
- [GitCode 配置](docs/GITCODE_SETUP.md)
- [指令参考](docs/COMMANDS.md)

## 开发

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
```

### 格式化代码

```bash
cargo fmt
```

## 贡献

欢迎贡献！请查看我们的贡献指南。

## 许可证

本项目采用 MIT OR Apache-2.0 双重许可。详见 LICENSE 文件。

## 致谢

- [crater](https://github.com/rust-lang/crater) - 原始 Rust 生态系统测试工具
- [crater-ohos](https://github.com/LuuuXXX/crater-ohos) - OpenHarmony 适配版本
