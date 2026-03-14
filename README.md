# Venue Rental System (Venue-Flow)

English | [中文](README-zh.md)

A modern backend architecture experiment implementing **Effect-Driven Design (EDD)** in a real system.

# Introduction

Venue-Flow is a backend service written in **Rust** that implements a venue rental platform while serving as an **experimental validation project for Effect-Driven Design (EDD)**.

Unlike typical demo projects, the primary goal of this repository is to **verify whether EDD can work in real engineering environments**, including:

- business orchestration
- side-effect isolation
- testability
- system evolution

The venue rental system itself is only the **domain used to validate the architecture**.

------

# Status

**Version:** `0.2.0-alpha`

The project has reached the first milestone of **EDD architecture validation**.

Current state:

- Core business features are fully implemented
- The **Pure Layer (core/domain/app)** architecture is stable
- Business logic is fully functional
- API functionality is usable
- Unit testing for business logic is implemented

However, several engineering components are still under integration:

- Integration tests are currently executed manually via API tools and `curl`
- `k6` performance tests exist but are not yet integrated into the repository
- Event runtime and async worker system are partially implemented

This release focuses on validating the **architectural feasibility of Effect-Driven Design**, rather than production readiness.

------

# What is Effect-Driven Design (EDD)

Effect-Driven Design is an architectural methodology centered around **identifying and organizing side effects**.

Traditional architectures often mix business logic with external operations such as:

- database writes
- network calls
- logging
- message publishing

EDD proposes a different approach:

> **Business logic should make decisions.
> Side effects should execute those decisions.**

The architecture separates **decision logic** from **effect execution**.

Core principle:

```
Business logic → declares effects
Infrastructure → executes effects
```

This separation enables:

- deterministic business logic
- easier testing
- clearer system evolution

------

# What This Project Demonstrates

This repository validates several important hypotheses about EDD.

### 1. Fully Testable Business Logic

By isolating side effects, the pure layers can achieve near **100% deterministic testing**.

### 2. Explicit Side-Effect Modeling

All side effects are represented as **events** returned by UseCases.

Example:

```rust
Outcome {
    data: user,
    from_case: AppUseCase::RegisterUser,
    events: vec![
        AppEvent::SendWelcomeEmail { email },
        AppEvent::AssignABTag { user_id }
    ]
}
```

UseCases describe **what should happen**, not **how it happens**.

------

### 3. Trait-based Side Effect Abstraction

External operations are abstracted via traits.

Example:

```rust
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<()>;
}
```

This enables:

- dependency injection
- mocking
- infrastructure swapping

------

### 4. Event-Driven Side Effect Execution

External side effects are decoupled from business logic using events.

```
UseCase → Outcome → Event → Worker → External Systems
```

Benefits:

- better resilience
- clearer failure handling
- async execution capability

------

### 5. Architecture Evolvability

New side effects can be added without modifying existing business logic.

------

# Architecture Overview

EDD organizes systems by **side-effect boundaries**.

```
core → domain → app → event → infra
```

Layer responsibilities:

| Layer  | Responsibility                      | Side Effect Level |
| ------ | ----------------------------------- | ----------------- |
| core   | Entities, value objects, validation | 0                 |
| domain | Behavior contracts (traits, DTOs)   | 1                 |
| app    | UseCase orchestration               | 2                 |
| event  | Event dispatching and scheduling    | 2-3               |
| infra  | Real side-effect implementations    | 3                 |

# Core Domain Features

The system models a venue rental platform.

### User Management

- user registration
- login
- contact update
- role management

### Venue Management

- publish venue
- edit venue information
- manage venue status
- list venues

### Rental Workflow

- browse available venues
- submit rental requests
- accept / reject rental applications

# Testing Strategy

EDD enables **layer-aligned testing strategies**.

| Layer  | Testing Method          |
| ------ | ----------------------- |
| core   | pure unit tests         |
| domain | contract tests          |
| app    | mocked UseCase tests    |
| event  | event schema validation |
| infra  | integration tests       |

Pure layers (`core/domain/app`) can be tested **without external dependencies**.

Example:

```rust
#[tokio::test]
async fn test_register_user() {
    let repo = MockUserRepository::new();
    let result = register_user(input, &repo).await;

    assert!(result.is_ok());
}
```

------

# Performance Testing

This project uses `k6` not only for throughput and latency testing, but also to validate an important architectural assumption of **Effect-Driven Design (EDD)**:

> business correctness should remain stable under pressure, while the architectural overhead should stay within an acceptable range.

The current pressure-test results show two key conclusions:

- **No business-state violations were observed in the tested state-machine scenarios**  
  Under concurrent pressure, invalid transitions were consistently rejected, and the business flow remained stable.  
  This suggests that EDD successfully keeps business rules constrained inside the pure orchestration layer instead of letting correctness depend on side-effect execution order.

- **The architectural cost of EDD appears acceptable for this project stage**  
  Although bottlenecks still appear in write-heavy paths, the system remains functionally usable, and the main pressure is concentrated at the side-effect boundary rather than in business logic itself.  
  In other words, EDD did not introduce a disproportionate performance penalty relative to the clarity and correctness guarantees it provides.

### Conclusion

At the current stage, the performance test provides an initial practical validation of EDD:

- it helps preserve business correctness under pressure
- it makes bottlenecks easier to localize
- it does so without causing significant performance loss at the current system scale

### Future plans include

- automated performance benchmarks
- architecture comparison (EDD vs traditional layering)

# Quick Start

### Requirements

```
Rust
PostgreSQL
Redis
```

### Clone Repository

```
git clone https://github.com/webnew-b/venue-flow.git
cd venue-flow
```

### Configure Environment

```
cp .env.example .env
```

### Run Migrations

```
cargo run --bin migrate
```

### Run Server

```
cd run/
bash run.sh
```

### Run Tests

```
cargo test
```

# Contributing

This repository is currently an **architecture experiment project** and is not open for external contributions.

Issues and discussions are welcome.

# License

Apache License 2.0

------

# Author

Maintained by **Lexon**
