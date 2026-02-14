# Docker 部署指南

本文档介绍如何使用 Docker 部署 Rustling HTTP 服务器。

---

## 快速开始

### 使用 Docker Compose（推荐）

```bash
# 1. 构建并启动服务
docker-compose up -d

# 2. 查看日志
docker-compose logs -f

# 3. 测试服务
curl http://localhost:8080/health

# 4. 停止服务
docker-compose down
```

### 使用 Docker 命令

```bash
# 1. 构建镜像
docker build -t rustling-server:latest .

# 2. 运行容器
docker run -d \
  --name rustling-server \
  -p 8080:8080 \
  -e RUST_LOG=info \
  rustling-server:latest

# 3. 查看日志
docker logs -f rustling-server

# 4. 停止容器
docker stop rustling-server
docker rm rustling-server
```

---

## 镜像优化

### 多阶段构建

Dockerfile 使用多阶段构建以优化镜像大小：

- **Builder 阶段**: 使用 `rust:1.70-slim` 编译应用
- **Runtime 阶段**: 使用 `debian:bookworm-slim` 运行应用

### 镜像大小

| 阶段 | 大小 |
|------|------|
| Builder | ~1.5 GB |
| **Runtime** | **~80 MB** |

目标：通过进一步优化达到 <50MB（使用 alpine 或 distroless）

---

## 配置选项

### 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `RUST_LOG` | `info` | 日志级别 (trace/debug/info/warn/error) |
| `BIND_ADDRESS` | `0.0.0.0:8080` | 监听地址和端口 |
| `RELOAD_API_KEY` | - | 配置重载 API key（可选） |
| `RUST_BACKTRACE` | `1` | Rust 回溯跟踪 |

### 自定义环境变量

```bash
# 1. 复制示例文件
cp .env.example .env

# 2. 编辑 .env 文件
vim .env

# 3. 使用自定义配置启动
docker-compose up -d
```

---

## 健康检查

### 内置健康检查

Docker 容器包含自动健康检查：

```dockerfile
HEALTHCHECK --interval=30s \
            --timeout=3s \
            --start-period=5s \
            --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1
```

### 检查容器健康状态

```bash
# 查看健康状态
docker ps

# 查看详细健康检查日志
docker inspect rustling-server | grep -A 10 Health
```

健康状态说明：
- `starting` - 启动中（start-period 内）
- `healthy` - 健康
- `unhealthy` - 不健康（3 次重试失败后）

---

## 开发模式

### 使用开发配置

```bash
# 使用开发配置启动（包含 debug 日志）
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up

# 重新构建
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build
```

### 开发配置特性

- Debug 日志级别
- 预设 API key：`dev-secret-key`
- 挂载 rules 目录（只读）
- 不自动重启

---

## 生产部署

### 资源限制

docker-compose.yml 中已配置资源限制：

```yaml
deploy:
  resources:
    limits:
      cpus: '2'
      memory: 512M
    reservations:
      cpus: '0.5'
      memory: 128M
```

### 日志管理

自动日志轮转配置：

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

### 安全配置

1. **非 root 用户运行**
   - 容器内使用 `rustling` 用户（UID 1000）

2. **API Key 认证**
   - 设置 `RELOAD_API_KEY` 环境变量
   - 用于 `/config/reload` 端点

3. **只读文件系统（可选）**
   ```yaml
   read_only: true
   tmpfs:
     - /tmp
   ```

---

## API 端点

容器启动后，以下端点可用：

| 端点 | 方法 | 说明 | 认证 |
|------|------|------|------|
| `/health` | GET | 健康检查 | 否 |
| `/config/status` | GET | 配置状态 | 否 |
| `/parse` | POST | 解析文本 | 否 |
| `/parse/batch` | POST | 批量解析 | 否 |
| `/config/reload` | POST | 重载配置 | API Key |
| `/swagger-ui/` | GET | API 文档 | 否 |

### 示例请求

```bash
# 健康检查
curl http://localhost:8080/health

# 解析文本
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"text": "5 minutes"}'

# 批量解析
curl -X POST http://localhost:8080/parse/batch \
  -H "Content-Type: application/json" \
  -d '{"texts": ["5 minutes", "3 hours", "tomorrow"]}'

# 查看 API 文档
open http://localhost:8080/swagger-ui/

# 重载配置（需要 API key）
curl -X POST http://localhost:8080/config/reload \
  -H "X-API-Key: your-secret-key"
```

---

## 故障排查

### 容器无法启动

```bash
# 查看容器日志
docker-compose logs rustling

# 查看详细错误
docker logs rustling-server

# 检查端口占用
lsof -i :8080
```

### 健康检查失败

```bash
# 进入容器
docker exec -it rustling-server bash

# 手动测试健康检查
curl http://localhost:8080/health

# 检查进程
ps aux | grep rustling
```

### 构建失败

```bash
# 清理缓存重新构建
docker-compose build --no-cache

# 检查 Rust 版本
docker run --rm rust:1.70-slim rustc --version
```

---

## 高级配置

### 自定义规则目录

```yaml
# docker-compose.yml
services:
  rustling:
    volumes:
      - ./custom-rules:/app/rules:ro
```

### 多实例部署

```bash
# 启动多个实例
docker-compose up --scale rustling=3

# 使用 nginx 负载均衡
# 见 docs/nginx.conf.example
```

### Prometheus Metrics（未来）

```yaml
# docker-compose.yml
services:
  rustling:
    ports:
      - "8080:8080"  # HTTP API
      - "9090:9090"  # Metrics endpoint
```

---

## 镜像发布

### 构建优化镜像

```bash
# 构建生产镜像
docker build -t rustling-server:v0.10.0 .

# 标记为 latest
docker tag rustling-server:v0.10.0 rustling-server:latest
```

### 推送到 Registry

```bash
# Docker Hub
docker tag rustling-server:latest username/rustling-server:latest
docker push username/rustling-server:latest

# 私有 Registry
docker tag rustling-server:latest registry.example.com/rustling-server:latest
docker push registry.example.com/rustling-server:latest
```

---

## 性能优化

### 编译优化

Dockerfile 已配置：
- Release 模式编译
- Strip debug symbols
- 静态链接（减少运行时依赖）

### 进一步优化（TODO）

1. **使用 Alpine Linux**
   ```dockerfile
   FROM alpine:3.18
   RUN apk add --no-cache ca-certificates
   ```
   预期镜像大小：~30-40MB

2. **使用 Distroless**
   ```dockerfile
   FROM gcr.io/distroless/cc-debian12
   ```
   预期镜像大小：~20-30MB

3. **使用 UPX 压缩**
   ```dockerfile
   RUN upx --best --lzma /usr/local/bin/rustling-server
   ```
   预期压缩：50-60%

---

## 相关文档

- [HTTP Server Example](../examples/http_server.rs) - 服务器示例代码
- [Phase 2 Status](./plans/PHASE2_STATUS.md) - HTTP 服务器实现状态
- [Project Roadmap](./plans/PROJECT_ROADMAP_V3.md) - 项目路线图

---

## 常见问题

### Q: 如何修改监听端口？

A: 修改 `docker-compose.yml` 中的端口映射：
```yaml
ports:
  - "3000:8080"  # 主机端口:容器端口
```

### Q: 如何启用 Apollo 配置？

A: 需要编译时启用 `apollo` feature：
```dockerfile
RUN cargo build --release --features apollo --example http_server
```

### Q: 镜像太大怎么办？

A: 当前镜像 ~80MB，可以通过以下方式优化：
1. 使用 Alpine 基础镜像
2. 使用 Distroless 镜像
3. 使用 UPX 压缩二进制文件

### Q: 如何查看容器内的文件？

A:
```bash
docker exec -it rustling-server sh
ls -la /app
```

---

**文档维护者**: Claude Code
**最后更新**: 2026-02-14
**版本**: 1.0
