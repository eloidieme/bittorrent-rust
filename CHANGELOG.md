# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Comprehensive GitHub Actions CI/CD pipeline
- Automated dependency updates with Dependabot
- Release automation workflow
- Security audit integration
- Multi-platform testing (Linux, Windows, macOS)
- Documentation generation and hosting
- Code coverage reporting
- Performance benchmarking framework

### Changed

- Improved project organization with proper directory structure
- Enhanced Makefile with comprehensive development commands
- Updated README with better documentation and usage examples

## [0.1.0] - 2024-09-02

### Added

- Initial implementation of BitTorrent client library
- Bencode parsing engine with comprehensive test coverage
- Torrent file parsing support (single-file and multi-file torrents)
- SHA-1 info hash computation
- HTTP tracker communication and peer discovery
- Basic peer protocol implementation with handshake
- Peer ID generation following BitTorrent specification
- Example basic client demonstrating core functionality
- Comprehensive error handling with detailed error types
- Zero-copy parsing for efficient memory usage
- Integration tests and unit tests
- Documentation and development notes

### Technical Details

- **Bencode Parser**: Complete implementation supporting strings, integers, lists, and dictionaries
- **Torrent Support**: Both single-file and multi-file torrent parsing
- **Tracker Protocol**: HTTP tracker announce requests with proper parameter encoding
- **Peer Protocol**: TCP connection management and BitTorrent handshake
- **Error Handling**: Structured error types with byte offset tracking
- **Testing**: 42+ unit tests covering edge cases and error conditions

### Dependencies

- `serde` - Serialization framework
- `serde_json` - JSON serialization
- `thiserror` - Error handling utilities
- `sha1` - SHA-1 hashing
- `reqwest` - HTTP client for tracker communication
- `urlencoding` - URL parameter encoding
- `url` - URL parsing and manipulation
- `bytes` - Byte buffer utilities

## [0.0.1] - 2024-09-01

### Added

- Initial project setup
- Basic project structure
- Cargo.toml configuration
- Initial development notes
