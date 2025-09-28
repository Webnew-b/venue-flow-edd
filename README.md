# Venue Rental System (Venue-Flow)

A modern venue rental platform built on **Effect-Driven Design (EDD)** architectural principles.

## Project Overview

Venue-Flow is a venue rental platform that connects venue lessors with event organizers. More importantly, this is an **experimental project to validate the feasibility of Effect-Driven Design (EDD) in real engineering projects**.

EDD proposes an architectural philosophy centered on "identification and organization of side effects", building more maintainable and testable systems through systematic identification, isolation, and management of side effects. This project validates and explores the following core EDD concepts through implementing a complete venue rental system:

- **Side Effect Layered Isolation**: Verifying the practicality of layered architecture through clear separation between core/domain/app pure layers and infra/event side effect layers
- **Outcome Pattern**: All UseCases return a unified result+events structure, validating the clarity of this pattern for business expression
- **Trait Abstraction**: Abstracting side effects through traits, validating support for testing and dependency injection
- **Event-Driven Side Effects**: Decoupling external side effects through event mechanisms, validating improvements in system resilience and scalability
- **100% Testable Business Logic**: Through pure layer design, validating the feasibility of fully testable business logic

For more details, please refer to: [EDD Effect-Driven Design System Documentation]()

### Core Features

- **User Management**: Registration, login, role management (organizer/lessor)
- **Venue Management**: Publishing, editing, status management
- **Rental Process**: Browse venues, submit applications, process requests
- **Event-Driven**: Asynchronous processing of notifications, status changes and other side effects

## Architecture Design

This project strictly follows the EDD layered architecture:

```
src/
├── core/           # Pure business logic layer - No side effects, 100% testable
├── domain/         # Behavior definition layer - trait interfaces and DTO definitions
├── app/            # Business orchestration layer - UseCase composition and event generation
├── infra/          # Side effect implementation layer - Database, API, Web handlers and other concrete implementations
└── event/          # Event processing layer - Asynchronous side effect scheduling
```

### Layer Responsibilities

| Layer | Status | Responsibilities | Side Effect Level |
|-------|--------|-----------------|-------------------|
| **core** | Complete | Entity definitions, value objects, business validation rules | 0 (Pure functions) |
| **domain** | Mostly Complete | Repository/Service trait definitions, DTO structures | 1 (Abstract definitions) |
| **app** | Complete | UseCase implementations, business process orchestration, event generation | 2 (Composition logic) |
| **infra** | In Progress | SeaORM database implementation, Redis cache, external APIs, Web handlers | 3 (Real side effects) |
| **event** | In Progress | Event consumers, Outbox pattern, asynchronous processing | 3 (Trigger side effects) |

## Current Progress

### Completed Modules

#### Core Layer
- [x] User Entity
- [x] Venue Entity
- [x] RentalRequest Entity
- [x] Value object definitions (Email, UserId, VenueId, etc.)
- [x] Business validation rules

#### Domain Layer
- [x] UserRepository trait
- [x] VenueRepository trait
- [x] RentalRequestRepository trait
- [x] Various DTO definitions
- [ ] Service trait improvements (partially pending)

#### App Layer - UseCase Implementations
- [x] `RegisterUser` - User registration
- [x] `LoginUser` - User login
- [x] `UpdateUserContact` - Update contact information
- [x] `CreateVenue` - Publish venue
- [x] `UpdateVenueStatus` - Manage venue status
- [x] `ListMyVenues` - View my venues
- [x] `UpdateVenueDetails` - Edit venue information
- [x] `ListAvailableVenues` - Browse available venues
- [x] `GetVenueDetails` - View venue details
- [x] `SubmitRentalRequest` - Submit rental application
- [x] `ListVenueRentalRequests` - View rental requests
- [x] `AcceptRentalRequest` - Accept rental request
- [x] `RejectRentalRequest` - Reject rental request

### In Development

#### Infra Layer
- [x] Service implementation infrastructure
- [ ] Repository concrete implementations
  - [ ] UserRepository (SeaORM)
  - [ ] VenueRepository (SeaORM)
  - [ ] RentalRequestRepository (SeaORM)
- [ ] Web handlers (Actix-Web)
  - [ ] User authentication endpoints
  - [ ] Venue management endpoints
  - [ ] Rental request endpoints
- [ ] External API integrations
  - [ ] Email service
  - [ ] SMS service
- [ ] Redis cache layer

#### Event Layer
- [ ] Event definition refinement
- [ ] Event consumer implementations
  - [ ] UserRegistered consumer
  - [ ] VenueCreated consumer
  - [ ] RentalRequestSubmitted consumer
  - [ ] RentalRequestAccepted consumer
  - [ ] RentalRequestRejected consumer
- [ ] Outbox pattern implementation
- [ ] Event retry mechanism

### To Be Developed

- [ ] Integration test suite (planned refactoring)
- [ ] Docker deployment configuration
- [ ] Performance optimization
- [ ] Monitoring and logging system

## Tech Stack

- **Language**: Rust
- **Web Framework**: Actix-Web
- **ORM**: SeaORM
- **Database**: PostgreSQL
- **Cache**: Redis
- **Async Runtime**: Tokio

## Project Structure

```
TODO: Detailed project structure to be organized
```

## Quick Start

### Environment Requirements

```
TODO: Environment configuration instructions to be added
```

### Installation Steps

```bash
# Clone the project
git clone https://github.com/yourusername/venue-flow.git
cd venue-flow

# Install dependencies
cargo build

# Configure database
cp .env.example .env
# Edit .env file, configure database connection

# Run database migrations
cargo run --bin migrate

# Run tests
cargo test

# Start development server
cargo run
```

## Testing Strategy

Following EDD testing guidelines for multi-layered testing strategy:

- **Core Layer**: 100% unit test coverage
- **Domain Layer**: Mock trait behavior testing
- **App Layer**: UseCase integration testing with mock dependencies
- **Infra Layer**: Integration testing using testcontainers
- **Event Layer**: Event serialization and processing logic testing

> Note: Test code is being planned for refactoring to better reflect EDD testing philosophy

## EDD Practice Points

Key EDD concepts validated in this project's development:

1. **Side Effect Identification and Isolation**: All side effects are decoupled through trait abstraction or event mechanisms
2. **Pure Layer Design**: core/domain/app three layers form the pure layer, ensuring business logic testability
3. **Outcome Pattern**: All UseCases return a unified `Outcome<T>` structure containing result data and pending events
4. **Event-Driven Side Effects**: Decoupling external side effects from main flow through event mechanisms, improving system resilience
5. **Dependency Injection Strategy**: Using HRTB and Context patterns to handle complex dependency relationships

## Contributing

This project is currently under construction and not open for PR merging. If you have suggestions or questions, please submit an Issue for discussion.

## License

MIT License

## Contact

For questions or suggestions, please submit an Issue or contact the project maintainer.

---

*This project adopts the Effect-Driven Design (EDD) methodology, dedicated to exploring and validating EDD best practices in real projects.*
