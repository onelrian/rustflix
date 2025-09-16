# RustFlix - High-Performance Media Server Architecture

## Project Overview

RustFlix is a high-performance, Rust-based media server designed to meet or exceed Jellyfin's functionality while providing superior performance, modularity, and maintainability.

## Vision & Goals

### Primary Goals
- **Performance**: 10x faster startup times, 50% lower memory usage, 2x higher concurrent stream capacity
- **Reliability**: Zero-downtime updates, graceful error handling, automatic recovery
- **Scalability**: Horizontal scaling support, microservice-ready architecture
- **Maintainability**: Clean modular design, comprehensive testing, excellent documentation

### Performance Targets
- **Startup Time**: < 2 seconds (vs Jellyfin's ~20 seconds)
- **Memory Usage**: < 512MB base (vs Jellyfin's ~1GB)
- **Concurrent Streams**: 100+ simultaneous streams per core
- **API Response Time**: < 50ms for metadata queries
- **Transcoding Latency**: < 3 seconds to first segment
- **Database Query Time**: < 10ms for common operations

## System Architecture

### Core Components

#### 1. Media Library Manager (`media_library`)
**Responsibilities:**
- File system scanning and monitoring
- Media file detection and validation
- Library organization and indexing
- Change detection and incremental updates

**Key Features:**
- Async file system traversal using `tokio::fs`
- Efficient change detection with file system watchers
- Parallel processing of media files
- Smart duplicate detection

#### 2. Metadata Provider System (`metadata`)
**Responsibilities:**
- External metadata source integration (TMDb, OMDb, TVDb)
- Metadata caching and persistence
- Image/artwork management
- Metadata enrichment and normalization

**Key Features:**
- Plugin-based provider system
- Intelligent metadata matching algorithms
- Async HTTP client pool for external APIs
- Image processing and optimization

#### 3. Streaming Engine (`streaming`)
**Responsibilities:**
- Direct play streaming
- Adaptive bitrate streaming (HLS, DASH)
- Real-time transcoding pipeline
- Hardware acceleration integration

**Key Features:**
- Zero-copy streaming where possible
- FFmpeg integration with hardware acceleration
- Segment caching and CDN support
- Quality adaptation algorithms

#### 4. Database Layer (`database`)
**Responsibilities:**
- Media metadata persistence
- User data and preferences
- Playback state tracking
- Configuration storage

**Technology Stack:**
- **Primary**: PostgreSQL for relational data
- **Cache**: Redis for session data and frequently accessed metadata
- **Search**: Embedded search index for fast text queries

#### 5. Authentication & Authorization (`auth`)
**Responsibilities:**
- User account management
- JWT token handling
- Role-based access control
- API key management

**Key Features:**
- Secure password hashing (Argon2)
- OAuth2 integration support
- Session management with Redis
- Fine-grained permissions

#### 6. Plugin System (`plugins`)
**Responsibilities:**
- Dynamic plugin loading
- Plugin lifecycle management
- Sandboxed execution environment
- Plugin API versioning

**Key Features:**
- WebAssembly-based plugin runtime
- Secure plugin isolation
- Hot-reloading capabilities
- Plugin dependency management

#### 7. API Gateway (`api`)
**Responsibilities:**
- REST API endpoints
- WebSocket connections for real-time updates
- Request routing and middleware
- Rate limiting and throttling

**Technology Stack:**
- **HTTP Server**: Axum for high-performance async HTTP
- **WebSocket**: Native tokio-tungstenite
- **Serialization**: serde with MessagePack for binary APIs

#### 8. Configuration Manager (`config`)
**Responsibilities:**
- Configuration file management
- Environment variable handling
- Runtime configuration updates
- Configuration validation

#### 9. Monitoring & Observability (`monitoring`)
**Responsibilities:**
- Metrics collection and export
- Structured logging
- Health checks and diagnostics
- Performance profiling

**Technology Stack:**
- **Metrics**: Prometheus metrics
- **Logging**: tracing with structured output
- **Health**: Custom health check framework

## Data Models

### Core Entities

```rust
// Media Library
struct MediaItem {
    id: Uuid,
    path: PathBuf,
    file_size: u64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    media_type: MediaType,
    metadata: MediaMetadata,
}

// Metadata
struct MediaMetadata {
    title: String,
    description: Option<String>,
    release_date: Option<NaiveDate>,
    genres: Vec<String>,
    cast: Vec<Person>,
    crew: Vec<CrewMember>,
    external_ids: HashMap<String, String>,
}

// User Management
struct User {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    roles: Vec<Role>,
    preferences: UserPreferences,
    created_at: DateTime<Utc>,
}
```

## Database Schema

### Tables Structure

#### Media Tables
- `media_items`: Core media file information
- `metadata`: Rich metadata for media items
- `collections`: Movies, TV shows, seasons, episodes hierarchy
- `genres`: Genre taxonomy
- `people`: Actors, directors, writers
- `images`: Artwork, posters, backdrops

#### User Tables
- `users`: User accounts and authentication
- `user_preferences`: Personalization settings
- `playback_state`: Resume positions and watch history
- `user_ratings`: User ratings and favorites

#### System Tables
- `libraries`: Media library configurations
- `plugins`: Installed plugin information
- `jobs`: Background task queue
- `audit_log`: System activity tracking

### Indexing Strategy
- **Primary Keys**: UUID v4 for all entities
- **Search Indexes**: Full-text search on titles, descriptions
- **Performance Indexes**: Composite indexes on frequently queried columns
- **Partitioning**: Time-based partitioning for logs and metrics

## API Design

### REST Endpoints

#### Media Library API
```
GET    /api/v1/libraries                    # List all libraries
POST   /api/v1/libraries                    # Create new library
GET    /api/v1/libraries/{id}/items         # Get library items
POST   /api/v1/libraries/{id}/scan          # Trigger library scan

GET    /api/v1/items/{id}                   # Get item details
GET    /api/v1/items/{id}/metadata          # Get item metadata
PUT    /api/v1/items/{id}/metadata          # Update metadata
```

#### Streaming API
```
GET    /api/v1/items/{id}/stream            # Direct play stream
GET    /api/v1/items/{id}/hls               # HLS playlist
GET    /api/v1/items/{id}/dash              # DASH manifest
POST   /api/v1/items/{id}/transcode         # Request transcoding
```

#### User Management API
```
POST   /api/v1/auth/login                   # User authentication
POST   /api/v1/auth/logout                  # Session termination
GET    /api/v1/users/me                     # Current user info
PUT    /api/v1/users/me/preferences         # Update preferences
```

### WebSocket Events
- `library.scan.progress`: Library scanning updates
- `playback.state.changed`: Playback state synchronization
- `metadata.updated`: Real-time metadata updates
- `system.notification`: System-wide notifications

## Performance Optimization Strategies

### Memory Management
- **Zero-Copy Streaming**: Use `bytes::Bytes` for buffer sharing
- **Object Pooling**: Reuse expensive objects like HTTP clients
- **Lazy Loading**: Load metadata and images on-demand
- **Memory Mapping**: Use `mmap` for large file operations

### Concurrency & Parallelism
- **Async I/O**: Full async/await throughout the stack
- **Work Stealing**: Use `tokio` runtime with work-stealing scheduler
- **Parallel Processing**: Use `rayon` for CPU-intensive tasks
- **Connection Pooling**: Database and HTTP connection pools

### Caching Strategy
- **Multi-Level Caching**: Memory → Redis → Database
- **Cache Warming**: Preload frequently accessed data
- **Intelligent Eviction**: LRU with frequency-based adjustments
- **Cache Coherence**: Event-driven cache invalidation

### Hardware Acceleration
- **Video Decoding**: NVENC, VAAPI, VideoToolbox integration
- **Image Processing**: GPU-accelerated thumbnail generation
- **Crypto Operations**: Hardware AES for encryption

## Security Architecture

### Authentication Flow
1. User credentials → Argon2 password verification
2. JWT token generation with short expiry
3. Refresh token for seamless re-authentication
4. Session storage in Redis with automatic cleanup

### Authorization Model
- **Role-Based Access Control (RBAC)**
- **Resource-Level Permissions**
- **API Key Authentication** for external integrations
- **Rate Limiting** per user/IP

### Data Protection
- **Encryption at Rest**: Database encryption
- **Encryption in Transit**: TLS 1.3 for all communications
- **Secure Headers**: HSTS, CSP, X-Frame-Options
- **Input Validation**: Comprehensive request validation

## Plugin System Architecture

### Plugin Runtime
- **WebAssembly (WASM)**: Secure, sandboxed execution
- **Component Model**: Well-defined plugin interfaces
- **Resource Limits**: CPU, memory, and I/O constraints
- **Hot Reloading**: Update plugins without server restart

### Plugin Types
- **Metadata Providers**: Custom metadata sources
- **Authentication Providers**: OAuth, LDAP, etc.
- **Notification Providers**: Discord, Slack, email
- **Storage Providers**: Cloud storage backends

## Development Standards

### Code Quality
- **Error Handling**: Comprehensive error types with `thiserror`
- **Testing**: Unit tests (>90% coverage), integration tests
- **Documentation**: Inline docs, API documentation
- **Linting**: Clippy with strict settings

### Dependencies
- **Core Runtime**: `tokio` for async runtime
- **HTTP Server**: `axum` for web framework
- **Database**: `sqlx` for async database operations
- **Serialization**: `serde` with JSON/MessagePack
- **Logging**: `tracing` for structured logging
- **Configuration**: `config` crate for settings management

### Build & Deployment
- **Containerization**: Multi-stage Docker builds
- **Cross-Compilation**: Support for multiple architectures
- **CI/CD**: GitHub Actions for automated testing and deployment
- **Monitoring**: Prometheus metrics and Grafana dashboards

## Scalability Considerations

### Horizontal Scaling
- **Stateless Design**: All state in external stores (Redis, PostgreSQL)
- **Load Balancing**: Support for multiple server instances
- **Database Sharding**: Partition data across multiple databases
- **CDN Integration**: Offload static content delivery

### Microservice Readiness
- **Service Boundaries**: Clear separation of concerns
- **API Contracts**: Versioned APIs with backward compatibility
- **Event-Driven Architecture**: Async communication between services
- **Circuit Breakers**: Fault tolerance patterns

## Migration Strategy

### From Jellyfin
- **Database Migration Tools**: Automated data import
- **Configuration Mapping**: Convert Jellyfin settings
- **Plugin Compatibility**: Wrapper for existing plugins
- **Gradual Migration**: Side-by-side deployment support

## Monitoring & Observability

### Metrics Collection
- **System Metrics**: CPU, memory, disk, network usage
- **Application Metrics**: Request rates, response times, error rates
- **Business Metrics**: Active users, streaming sessions, library size
- **Custom Metrics**: Plugin-specific measurements

### Logging Strategy
- **Structured Logging**: JSON format with consistent fields
- **Log Levels**: Configurable verbosity levels
- **Log Aggregation**: Centralized log collection
- **Log Retention**: Configurable retention policies

### Health Checks
- **Liveness Probes**: Basic server responsiveness
- **Readiness Probes**: Service dependency checks
- **Deep Health Checks**: Database connectivity, external services
- **Performance Benchmarks**: Automated performance regression detection

## Version History

- **v0.1.0**: Initial architecture document
- **Last Updated**: 2025-09-16
- **Next Review**: 2025-10-16
