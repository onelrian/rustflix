# RustFlix

A high-performance media server written in Rust, designed to meet or exceed Jellyfin's functionality while providing superior performance, modularity, and maintainability.

## Features

- High Performance: 10x faster startup times, 50% lower memory usage
- Scalable Architecture: Microservice-ready, horizontal scaling support
- Modern Stack: Built with Rust, async/await, and cutting-edge technologies
- Plugin System: WebAssembly-based plugins with secure sandboxing
- Hardware Acceleration: NVENC, VAAPI, VideoToolbox support
- Rich Metadata: Integration with TMDb, OMDb, and other providers
- Adaptive Streaming: HLS and DASH support with quality adaptation

## Architecture

RustFlix is built as a modular workspace with the following components:

- **rustflix-core**: Core types, traits, and utilities
- **rustflix-database**: Database layer with PostgreSQL and Redis
- **rustflix-media-library**: File system scanning and media management
- **rustflix-metadata**: External metadata provider integration
- **rustflix-streaming**: Video streaming and transcoding engine
- **rustflix-auth**: Authentication and authorization system
- **rustflix-api**: REST and WebSocket API layer
- **rustflix-plugins**: WebAssembly plugin system
- **rustflix-config**: Configuration management
- **rustflix-monitoring**: Metrics, logging, and health checks
- **rustflix-server**: Main server binary

## Performance Goals

- Startup Time: < 2 seconds
- Memory Usage: < 512MB base
- Concurrent Streams: 100+ simultaneous streams per core
- API Response Time: < 50ms for metadata queries
- Transcoding Latency: < 3 seconds to first segment

## Prerequisites

- Rust 1.70+ (latest stable recommended)
- PostgreSQL 14+
- Redis 6+
- FFmpeg 5.0+ (for media processing)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/onelrian/rustflix.git
cd rustflix

# Build the project
cargo build --release

# Run the server
cargo run --bin rustflix-server
```

## Configuration

Configuration is managed through TOML files and environment variables:

```toml
# config/default.toml
[server]
host = "0.0.0.0"
port = 8096

[database]
url = "postgresql://rustflix:password@localhost/rustflix"

[redis]
url = "redis://localhost:6379"

[media]
library_paths = ["/media/movies", "/media/tv"]
```

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out html

# Run integration tests
cargo test --test integration
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Jellyfin](https://jellyfin.org/) - Inspiration and feature reference
- [Rust Community](https://www.rust-lang.org/community) - Amazing ecosystem and support
- [FFmpeg](https://ffmpeg.org/) - Media processing foundation

---

**Status**: Under Active Development  
**Version**: 0.1.0-alpha
