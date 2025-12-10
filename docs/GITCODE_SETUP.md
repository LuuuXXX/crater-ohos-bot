# GitCode 配置指南

本指南将帮助你在 GitCode 上配置 crater-ohos-bot。

## 前置准备

- 一个 GitCode 账户
- 管理员权限的仓库（用于配置 Webhook）
- 已部署的 crater-ohos-bot 实例（需要有公网 URL）

## 步骤 1: 获取个人访问令牌

### 1.1 进入设置页面

1. 登录 GitCode
2. 点击右上角头像
3. 选择 "设置" (Settings)
4. 在左侧菜单选择 "个人访问令牌" (Access Tokens)

### 1.2 创建新令牌

1. 点击 "生成新令牌" (Generate new token)
2. 填写令牌描述，例如: `crater-bot-token`
3. 选择过期时间（建议选择较长时间或永不过期）
4. 勾选以下权限:
   - `api` - 完整 API 访问权限
   - `write_repository` - 写入仓库权限（用于发布评论）
5. 点击 "生成令牌" (Generate token)
6. **重要**: 复制生成的令牌并妥善保存（此令牌只显示一次）

### 1.3 配置到 Bot

将获取的令牌填入 `config.toml`:

```toml
[platforms.gitcode]
enabled = true
api_url = "https://gitcode.com/api/v5"
access_token = "your-gitcode-token-here"  # 填入刚才复制的令牌
webhook_secret = "your-webhook-secret"     # 稍后设置
```

## 步骤 2: 配置 Webhook

### 2.1 进入仓库设置

1. 打开你的 GitCode 仓库
2. 点击 "设置" (Settings)
3. 在左侧菜单选择 "Webhooks"

### 2.2 添加新 Webhook

1. 点击 "添加 Webhook" (Add Webhook)
2. 填写以下信息:

**URL**:
```
https://your-bot-domain.com/webhook/gitcode
```
替换 `your-bot-domain.com` 为你的 Bot 实例的公网域名。

**密钥令牌** (Secret Token):
```
your-webhook-secret
```
设置一个强密码作为 Webhook 密钥，用于验证请求来源。

**触发事件**:
- 勾选 "Note events" (评论事件)
- 取消勾选其他不需要的事件

**启用 SSL 验证**:
- 如果使用 HTTPS（强烈推荐），确保勾选 "启用 SSL 验证"

3. 点击 "添加 Webhook" (Add Webhook)

### 2.3 更新 Bot 配置

将 Webhook 密钥填入 `config.toml`:

```toml
[platforms.gitcode]
enabled = true
api_url = "https://gitcode.com/api/v5"
access_token = "your-gitcode-token"
webhook_secret = "your-webhook-secret"  # 填入刚才设置的密钥
```

### 2.4 重启 Bot

```bash
# systemd
sudo systemctl restart crater-ohos-bot

# Docker
docker restart crater-ohos-bot

# 直接运行
# 停止当前进程并重新启动
```

## 步骤 3: 测试配置

### 3.1 测试 Webhook

1. 在仓库设置的 Webhooks 页面
2. 找到刚创建的 Webhook
3. 点击 "测试" (Test)
4. 选择 "Note events"
5. 查看响应状态（应该返回 200 OK）

### 3.2 测试 Bot 指令

1. 在仓库中创建一个新 Issue
2. 在 Issue 中发表评论:
```
@crater-bot help
```
3. 等待几秒钟，Bot 应该会回复帮助信息

如果 Bot 成功回复，说明配置完成！

### 3.3 测试完整流程

尝试运行一个实验:

```
@crater-bot run stable beta
```

Bot 应该:
1. 回复确认消息
2. 创建实验
3. 实验完成后发布结果

## 常见问题排查

### Bot 没有响应

**检查清单**:

1. **Webhook 是否正确触发**:
   - 在 GitCode Webhook 设置页面查看"最近交付" (Recent Deliveries)
   - 检查请求状态码（应该是 200）
   - 查看请求和响应内容

2. **Bot 是否运行**:
   ```bash
   # systemd
   sudo systemctl status crater-ohos-bot
   
   # Docker
   docker ps | grep crater-ohos-bot
   
   # 健康检查
   curl http://your-bot-domain.com/health
   ```

3. **查看 Bot 日志**:
   ```bash
   # systemd
   sudo journalctl -u crater-ohos-bot -f
   
   # Docker
   docker logs -f crater-ohos-bot
   ```

4. **检查配置文件**:
   - 确认 access_token 正确
   - 确认 webhook_secret 与 GitCode 设置一致
   - 确认 trigger_prefix 与命令中使用的一致

### Webhook 验证失败

**错误信息**: "Webhook signature verification failed"

**解决方法**:
1. 检查 `config.toml` 中的 `webhook_secret` 与 GitCode Webhook 设置中的密钥令牌是否一致
2. 确保没有多余的空格或换行符
3. 重新配置 Webhook 密钥并更新配置

### Bot 无法发布评论

**错误信息**: "Failed to post comment to GitCode"

**可能原因**:
1. Access Token 无效或过期
   - 解决: 重新生成令牌
2. Access Token 权限不足
   - 解决: 确保勾选了 `api` 和 `write_repository` 权限
3. Bot 账户没有仓库访问权限
   - 解决: 确保 Bot 账户可以访问目标仓库

### 命令解析失败

**错误信息**: "未知命令" 或 "参数错误"

**检查**:
1. 确认触发前缀正确（默认 `@crater-bot`）
2. 命令拼写正确
3. 参数数量正确（如 `run` 命令需要至少 2 个工具链）

示例:
```
# 正确
@crater-bot run stable beta

# 错误 - 触发前缀不对
@crater run stable beta

# 错误 - 参数不足
@crater-bot run stable
```

## 权限要求

Bot 账户需要以下权限:

- **评论权限**: 能够在 Issue 中发布评论
- **读取权限**: 能够读取 Issue 信息
- **Webhook 接收**: 仓库管理员才能配置 Webhook

建议:
- 为 Bot 创建专用的 GitCode 账户
- 给予该账户仓库的 Developer 或更高权限
- 使用该账户的 Access Token

## 安全建议

1. **Access Token 安全**:
   - 不要将 Token 提交到代码仓库
   - 使用环境变量或加密的配置文件
   - 定期轮换 Token

2. **Webhook 密钥**:
   - 使用强密码（至少 32 个字符）
   - 不要与其他系统共享密钥
   - 定期更换密钥

3. **HTTPS**:
   - 生产环境必须使用 HTTPS
   - 配置有效的 SSL 证书
   - 启用 SSL 验证

4. **网络安全**:
   - 限制 Bot 服务器的入站连接
   - 使用防火墙规则
   - 考虑使用 IP 白名单

## 高级配置

### 多仓库支持

Bot 可以同时为多个仓库服务，只需:
1. 在每个仓库配置相同的 Webhook
2. 使用相同的配置文件
3. Bot 会根据 Webhook 中的仓库信息自动识别

### 自定义触发前缀

如果不想使用 `@crater-bot`，可以修改配置:

```toml
[bot]
name = "my-bot"
trigger_prefix = "@my-bot"
```

重启 Bot 后，使用新的前缀:
```
@my-bot run stable beta
```

### 日志级别调整

查看更详细的调试信息:

```bash
RUST_LOG=crater_ohos_bot=debug ./crater-ohos-bot
```

## 参考资料

- [GitCode API 文档](https://gitcode.com/help/api/v5/README.md)
- [GitCode Webhook 文档](https://gitcode.com/help/user/project/integrations/webhooks.md)
- [crater-ohos-bot 命令参考](COMMANDS.md)
