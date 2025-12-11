# 部署指南

## 环境要求

- **操作系统**: Linux (推荐 Ubuntu 20.04+) 或 macOS
- **Rust**: 1.70 或更高版本
- **内存**: 最低 512MB，推荐 1GB+
- **网络**: 需要能够访问 crater-ohos 服务和代码托管平台

## 从源码编译

### 1. 安装 Rust

如果还未安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. 克隆代码

```bash
git clone https://github.com/LuuuXXX/crater-ohos-bot.git
cd crater-ohos-bot
```

### 3. 编译

```bash
# 调试版本（开发使用）
cargo build

# 发布版本（生产使用）
cargo build --release
```

编译后的二进制文件位于：
- 调试版本: `target/debug/crater-ohos-bot`
- 发布版本: `target/release/crater-ohos-bot`

## 配置

### 1. 创建配置文件

```bash
cp config.example.toml config.toml
```

### 2. 编辑配置

编辑 `config.toml`，填入实际的配置信息：

```toml
[server]
host = "0.0.0.0"
port = 8080

[crater]
api_url = "http://your-crater-instance:3000"
api_token = "your-crater-api-token"
callback_base_url = "https://your-bot-domain.com"

[platforms.gitcode]
enabled = true
api_url = "https://gitcode.com/api/v5"
access_token = "your-gitcode-token"
webhook_secret = "your-webhook-secret"

[bot]
name = "crater-bot"
trigger_prefix = "@crater-bot"
default_mode = "build-and-test"
default_crate_select = "demo"
```

**重要配置项说明**:

- `crater.api_url`: crater-ohos 服务的地址
- `crater.api_token`: crater-ohos API 认证令牌
- `crater.callback_base_url`: Bot 的公网访问地址（用于接收回调）
- `platforms.gitcode.access_token`: GitCode 个人访问令牌
- `platforms.gitcode.webhook_secret`: GitCode Webhook 密钥

## 运行方式

### 方式一: 直接运行

```bash
./target/release/crater-ohos-bot
```

### 方式二: 使用 systemd（推荐）

1. 创建 systemd 服务文件 `/etc/systemd/system/crater-ohos-bot.service`:

```ini
[Unit]
Description=Crater OHOS Bot
After=network.target

[Service]
Type=simple
User=crater
WorkingDirectory=/opt/crater-ohos-bot
ExecStart=/opt/crater-ohos-bot/crater-ohos-bot
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

2. 创建专用用户（可选但推荐）:

```bash
sudo useradd -r -s /bin/false crater
```

3. 部署文件:

```bash
sudo mkdir -p /opt/crater-ohos-bot
sudo cp target/release/crater-ohos-bot /opt/crater-ohos-bot/
sudo cp config.toml /opt/crater-ohos-bot/
sudo chown -R crater:crater /opt/crater-ohos-bot
```

4. 启动服务:

```bash
sudo systemctl daemon-reload
sudo systemctl enable crater-ohos-bot
sudo systemctl start crater-ohos-bot
```

5. 查看状态:

```bash
sudo systemctl status crater-ohos-bot
sudo journalctl -u crater-ohos-bot -f
```

### 方式三: Docker 部署

1. 创建 Dockerfile:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/crater-ohos-bot /usr/local/bin/
WORKDIR /app
CMD ["crater-ohos-bot"]
```

2. 构建镜像:

```bash
docker build -t crater-ohos-bot:latest .
```

3. 运行容器:

```bash
docker run -d \
  --name crater-ohos-bot \
  -p 8080:8080 \
  -v $(pwd)/config.toml:/app/config.toml:ro \
  --restart unless-stopped \
  crater-ohos-bot:latest
```

4. 查看日志:

```bash
docker logs -f crater-ohos-bot
```

### 方式四: Docker Compose

创建 `docker-compose.yml`:

```yaml
version: '3.8'

services:
  crater-ohos-bot:
    build: .
    image: crater-ohos-bot:latest
    container_name: crater-ohos-bot
    ports:
      - "8080:8080"
    volumes:
      - ./config.toml:/app/config.toml:ro
    restart: unless-stopped
    environment:
      - RUST_LOG=crater_ohos_bot=info
```

运行:

```bash
docker-compose up -d
```

## 反向代理配置

### Nginx

```nginx
server {
    listen 80;
    server_name bot.example.com;

    # 重定向到 HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name bot.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Caddy

```
bot.example.com {
    reverse_proxy localhost:8080
}
```

## 环境变量

可以通过环境变量覆盖配置文件中的部分设置：

```bash
export RUST_LOG=crater_ohos_bot=debug
export CRATER_API_TOKEN=your-token
```

## 健康检查

Bot 提供健康检查端点：

```bash
curl http://localhost:8080/health
```

返回 `OK` 表示服务正常。

## 监控和日志

### 日志级别

通过 `RUST_LOG` 环境变量控制日志级别：

```bash
# 只显示错误
RUST_LOG=error

# 显示信息和错误
RUST_LOG=info

# 显示调试信息
RUST_LOG=debug

# 详细调试
RUST_LOG=crater_ohos_bot=trace
```

### 日志位置

- **直接运行**: 输出到标准输出
- **systemd**: 使用 `journalctl -u crater-ohos-bot`
- **Docker**: 使用 `docker logs crater-ohos-bot`

## 升级

### 系统服务方式

```bash
# 停止服务
sudo systemctl stop crater-ohos-bot

# 更新代码并重新编译
git pull
cargo build --release

# 更新二进制文件
sudo cp target/release/crater-ohos-bot /opt/crater-ohos-bot/

# 重启服务
sudo systemctl start crater-ohos-bot
```

### Docker 方式

```bash
# 拉取最新代码
git pull

# 重新构建镜像
docker build -t crater-ohos-bot:latest .

# 重启容器
docker-compose down
docker-compose up -d
```

## 故障排查

### 服务无法启动

1. 检查配置文件是否正确
2. 检查端口是否被占用: `lsof -i :8080`
3. 查看日志获取错误信息

### Webhook 不工作

1. 确认 Bot 可以从公网访问
2. 检查 Webhook 密钥是否匹配
3. 查看 GitCode Webhook 日志

### API 调用失败

1. 确认 crater-ohos 服务正常运行
2. 检查 API token 是否正确
3. 验证网络连接

## 安全建议

1. **使用 HTTPS**: 在生产环境必须使用 HTTPS
2. **保护配置文件**: 限制 config.toml 的读取权限
3. **定期更新**: 保持依赖和系统更新
4. **最小权限**: 使用专用用户运行服务
5. **防火墙**: 只开放必要的端口

## 性能调优

### 调整工作线程数

```bash
# 设置 Tokio 工作线程数
TOKIO_WORKER_THREADS=4 ./crater-ohos-bot
```

### 优化编译

```bash
# 使用 LTO 优化
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## 备份和恢复

目前实验映射存储在内存中，重启会丢失。未来版本将支持持久化存储。

建议定期备份：
- 配置文件 `config.toml`
- 日志文件（如果需要）
