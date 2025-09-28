# 场地租赁系统 (Venue-Flow)

[English](README.md) | 中文

基于**副作用驱动设计（Effect-Driven Design, EDD）**架构理念开发的现代化场地租赁平台。

## 项目概述

Venue-Flow 是一个场地租赁平台，连接场地租赁方和活动组织者。但更重要的是，这是一个**验证副作用驱动设计（EDD）在实际工程项目中可行性的实验项目**。

EDD 提出了"以副作用识别与组织为核心"的架构理念，通过将副作用系统化地识别、隔离和管理，来构建更加可维护、可测试的系统。本项目通过实现一个完整的场地租赁系统，在实践中验证和探索 EDD 的以下核心理念：

- **副作用分层隔离**：通过 core/domain/app 纯化层与 infra/event 副作用层的明确分离，验证分层架构的实用性
- **Outcome 模式**：所有 UseCase 返回统一的结果+事件结构，验证这种模式对业务表达的清晰度
- **Trait 抽象**：通过 trait 将副作用抽象化，验证其对测试和依赖注入的支持程度
- **事件驱动副作用**：将外部副作用通过事件机制解耦，验证系统韧性和可扩展性的提升
- **100% 可测试的业务逻辑**：通过纯化层设计，验证业务逻辑完全可测试的可行性

详情请参考：[EDD 副作用驱动设计体系文档]()

### 核心功能

- **用户管理**：注册、登录、角色管理（组织者/租赁人）
- **场地管理**：发布、编辑、状态管理
- **租赁流程**：浏览场地、提交申请、处理请求
- **事件驱动**：异步处理通知、状态变更等副作用

## 架构设计

本项目严格遵循 EDD 分层架构：

```
src/
├── core/           # 纯业务逻辑层 - 无副作用，100%可测试
├── domain/         # 行为定义层 - trait接口与DTO定义
├── app/            # 业务编排层 - UseCase组合与事件生成
├── infra/          # 副作用实现层 - 数据库、API、Web handlers等具体实现
└── event/          # 事件处理层 - 异步副作用调度
```

### 层级职责说明

| 层级 | 状态 | 职责说明 | 副作用等级 |
|------|------|----------|------------|
| **core** | 完成 | 实体定义、值对象、业务验证规则 | 0 (纯函数) |
| **domain** | 基本完成 | Repository/Service trait定义、DTO结构 | 1 (抽象定义) |
| **app** | 完成 | UseCase实现、业务流程编排、事件生成 | 2 (组合逻辑) |
| **infra** | 施工中 | SeaORM数据库实现、Redis缓存、外部API、Web handlers | 3 (真实副作用) |
| **event** | 施工中 | 事件消费者、Outbox模式、异步处理 | 3 (触发副作用) |

## 当前进度

### 已完成模块

#### Core层
- [x] 用户实体 (User Entity)
- [x] 场地实体 (Venue Entity)
- [x] 租赁请求实体 (RentalRequest Entity)
- [x] 值对象定义 (Email, UserId, VenueId等)
- [x] 业务验证规则

#### Domain层
- [x] UserRepository trait
- [x] VenueRepository trait
- [x] RentalRequestRepository trait
- [x] 各类DTO定义
- [ ] Service trait完善（部分待补充）

#### App层 - UseCase实现
- [x] `RegisterUser` - 用户注册
- [x] `LoginUser` - 用户登录
- [x] `UpdateUserContact` - 更新联系方式
- [x] `CreateVenue` - 发布场地
- [x] `UpdateVenueStatus` - 管理场地状态
- [x] `ListMyVenues` - 查看我的场地
- [x] `UpdateVenueDetails` - 编辑场地信息
- [x] `ListAvailableVenues` - 浏览可用场地
- [x] `GetVenueDetails` - 查看场地详情
- [x] `SubmitRentalRequest` - 提交租赁申请
- [x] `ListVenueRentalRequests` - 查看租赁请求
- [x] `AcceptRentalRequest` - 接受租赁请求
- [x] `RejectRentalRequest` - 拒绝租赁请求

### 正在开发

#### Infra层
- [x] Service实现基础架构
- [ ] Repository具体实现
  - [ ] UserRepository (SeaORM)
  - [ ] VenueRepository (SeaORM)
  - [ ] RentalRequestRepository (SeaORM)
- [ ] Web handlers (Actix-Web)
  - [ ] 用户认证接口
  - [ ] 场地管理接口
  - [ ] 租赁请求接口
- [ ] 外部API集成
  - [ ] 邮件服务
  - [ ] 短信服务
- [ ] Redis缓存层

#### Event层
- [ ] 事件定义完善
- [ ] 事件消费者实现
  - [ ] UserRegistered消费者
  - [ ] VenueCreated消费者
  - [ ] RentalRequestSubmitted消费者
  - [ ] RentalRequestAccepted消费者
  - [ ] RentalRequestRejected消费者
- [ ] Outbox模式实现
- [ ] 事件重试机制

### 待开发

- [ ] 集成测试套件（计划重构）
- [ ] Docker部署配置
- [ ] 性能优化
- [ ] 监控与日志系统

## 技术栈

- **语言**: Rust
- **Web框架**: Actix-Web
- **ORM**: SeaORM
- **数据库**: PostgreSQL
- **缓存**: Redis
- **异步运行时**: Tokio

## 项目结构

```
TODO: 详细项目结构待整理
```

## 快速开始

### 环境要求

```
TODO: 环境配置说明待补充
```

### 安装步骤

```bash
# 克隆项目
git clone https://github.com/yourusername/venue-flow.git
cd venue-flow

# 安装依赖
cargo build

# 配置数据库
cp .env.example .env
# 编辑.env文件，配置数据库连接

# 运行数据库迁移
cargo run --bin migrate

# 运行测试
cargo test

# 启动开发服务器
cargo run
```

## 测试策略

遵循 EDD 测试指南的多层次测试策略：

- **Core层**: 100% 单元测试覆盖
- **Domain层**: Mock trait 行为测试
- **App层**: UseCase 集成测试，使用 mock 依赖
- **Infra层**: 使用 testcontainers 的集成测试
- **Event层**: 事件序列化与处理逻辑测试

> 注意：测试代码正在计划重构，以更好地体现 EDD 的测试理念

## EDD 实践要点

本项目在实际开发中验证的 EDD 核心理念：

1. **副作用识别与隔离**：所有副作用都通过 trait 抽象或事件机制解耦
2. **纯化层设计**：core/domain/app 三层构成纯化层，确保业务逻辑可测试性
3. **Outcome 模式**：所有 UseCase 返回统一的 `Outcome<T>` 结构，包含结果数据和待处理事件
4. **事件驱动副作用**：通过事件机制将外部副作用与主流程解耦，提高系统韧性
5. **依赖注入策略**：采用 HRTB 和 Context 模式处理复杂依赖关系

## 贡献指南

本项目目前仍在施工中，暂不开放 PR 合并。如有建议或问题，欢迎提交 Issue 进行讨论。

## 许可证

Apache License 2.0

## 联系方式

如有问题或建议，请提交 Issue 或联系项目维护者。

---

*本项目采用副作用驱动设计（EDD）方法论，致力于探索和验证 EDD 在实际项目中的最佳实践。*
