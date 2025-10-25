# BuckyBank Indexer Docker Makefile

.PHONY: help build down logs clean test dev prod

# 默认目标
help:
	@echo "BuckyBank Indexer Docker 管理命令"
	@echo ""
	@echo "可用命令:"
	@echo "  build       构建开发环境镜像"
	@echo "  up          启动开发环境服务"
	@echo "  down        停止并移除开发环境容器"
	@echo "  logs        查看开发环境日志"

# 构建开发环境镜像
build:
	@echo "构建开发环境Docker镜像..."
	#docker-compose build
	docker build -t bucky_bank_indexer_solana .

push: build
	@echo "发布Docker镜像..."
	docker tag bucky_bank_indexer_solana rzexin/bucky_bank_indexer_solana:v1.0
	docker push rzexin/bucky_bank_indexer_solana:v1.0

# 启动开发环境服务
up:
	@echo "启动开发环境服务..."
	docker-compose up -d

# 停止开发环境服务
down:
	@echo "停止开发环境服务..."
	docker-compose down

# 查看开发环境日志
logs:
	@echo "查看开发环境日志..."
	docker-compose logs -f