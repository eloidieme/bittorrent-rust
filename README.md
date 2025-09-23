# BitTorrent Rust Client

[![CI](https://github.com/eloidieme/bittorrent-rust/workflows/CI/badge.svg)](https://github.com/eloidieme/bittorrent-rust/actions)
[![Crates.io](https://img.shields.io/crates/v/bittorrent-rust.svg)](https://crates.io/crates/bittorrent-rust)
[![Documentation](https://docs.rs/bittorrent-rust/badge.svg)](https://docs.rs/bittorrent-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A minimal BitTorrent client implementation in Rust, currently in development. This project implements the core BitTorrent protocol components including bencode parsing, torrent file handling, tracker communication, and peer protocol basics.

## 🚧 Current Status

**Work in Progress** - This is an educational project implementing BitTorrent protocol fundamentals. The client can currently:

- ✅ Parse bencoded data and torrent files
- ✅ Extract metainfo from .torrent files (single and multi-file torrents)
- ✅ Compute info hashes from torrent data
- ✅ Communicate with BitTorrent trackers via HTTP
- ✅ Perform basic peer handshakes
- ✅ Generate unique peer IDs

**Not yet implemented:**

- Piece downloading and verification
- Peer message handling (beyond handshake)
- File management and storage
- Piece selection algorithms
- Choking/unchoking logic
- End-to-end download functionality

## 🏗️ Architecture

The project is organized into several core modules:

### Core Modules

- **`bencode/`** - Bencode parsing engine for BitTorrent data format
- **`metainfo/`** - Torrent file parsing and validation
- **`infohash/`** - SHA-1 hash computation for torrent info sections
- **`announce/`** - Tracker communication and peer discovery
- **`peer_protocol/`** - Basic peer-to-peer connection handling

### Key Features

#### Bencode Parser

- Complete bencode format support (strings, integers, lists, dictionaries)
- Zero-copy parsing with lifetime management
- Comprehensive error handling with byte offset tracking

#### Torrent File Support

- Single-file torrents
- Multi-file torrents with directory structures
- Piece length validation
- SHA-1 piece hash verification

#### Tracker Communication

- HTTP tracker announce requests
- URL-encoded parameter handling
- Bencoded response parsing
- Peer list extraction

#### Peer Protocol

- TCP connection management
- BitTorrent handshake implementation
- Basic message sending/receiving
- Connection state tracking

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (uses 2024 edition)
- A .torrent file for testing

### Building

```bash
git clone <repository-url>
cd bittorrent-rust
cargo build --release
```

Or use the Makefile for convenience:

```bash
make build-release
```

### Running

Run the basic client example:

```bash
cargo run --example basic-client -- examples/torrents/debian.iso.torrent
```

Or use the Makefile:

```bash
make run-example
```

### Development Commands

The project includes a comprehensive Makefile with many useful commands:

```bash
make help          # Show all available commands
make quick         # Quick development check (format + lint + test)
make dev           # Development workflow (build + test + run example)
make ci            # CI pipeline (format check + clippy + tests)
make docs          # Generate and open documentation
make clean         # Clean build artifacts
```

See `make help` for the complete list of available commands.

## 📋 Usage Example

The current implementation demonstrates the basic BitTorrent workflow:

1. **Parse torrent file** - Extracts metainfo, computes info hash
2. **Generate peer ID** - Creates unique 20-byte peer identifier
3. **Contact tracker** - Sends announce request to discover peers
4. **Connect to peer** - Establishes TCP connection and performs handshake
5. **Send keep-alive** - Demonstrates basic message exchange

```rust
// Example from main.rs
let metainfo = Metainfo::from(&bencode_val)?;
let info_hash = compute_info_hash(&file)?;
let peer_id = generate_peer_id();

// Contact tracker
let tracker_resp = perform_announce(&client, metainfo.announce, announce_params)?;
let peers_list = PeersList::from(decoded_response)?;

// Connect to first peer
let mut peer_connection = PeerConnection::connect(addr, &info_hash, &peer_id)?;
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

The project includes comprehensive tests for:

- Bencode parsing edge cases
- Torrent file validation
- Error handling scenarios

## 📁 Project Structure

```
bittorrent-rust/
├── src/                    # Core library implementation
│   ├── lib.rs             # Library exports and peer ID generation
│   ├── bencode/           # Bencode parsing engine
│   │   ├── mod.rs         # Public API and Value enum
│   │   ├── parser.rs      # Core parsing logic
│   │   ├── cursor.rs      # Byte stream cursor
│   │   ├── error.rs       # Error types and handling
│   │   └── tests.rs       # Parser tests
│   ├── metainfo/          # Torrent file handling
│   │   ├── mod.rs         # Metainfo and Info structs
│   │   ├── helpers.rs     # Parsing utilities
│   │   └── error.rs       # Metainfo-specific errors
│   ├── infohash/          # SHA-1 hash computation
│   │   ├── mod.rs         # Info hash calculation
│   │   └── error.rs       # Hash-related errors
│   ├── announce/          # Tracker communication
│   │   └── announce.rs    # HTTP tracker requests
│   └── peer_protocol/     # Peer-to-peer protocol
│       ├── mod.rs         # Public API
│       ├── connection.rs  # TCP connection management
│       └── error.rs       # Protocol errors
├── examples/              # Example applications
│   ├── basic-client/      # Basic BitTorrent client example
│   │   ├── main.rs        # Example implementation
│   │   └── Cargo.toml     # Example dependencies
│   ├── torrents/          # Sample torrent files
│   │   └── debian.iso.torrent
│   ├── debug/             # Debug/temporary files
│   │   ├── parsed.json    # Parsed torrent data
│   │   └── peers.json     # Peer list from tracker
│   └── README.md          # Examples documentation
├── tests/                 # Integration tests
│   └── integration/       # End-to-end tests
│       └── basic_workflow.rs
├── docs/                  # Documentation
│   ├── DEVELOPMENT.md     # Development notes
│   └── notes.md           # Original development notes
├── Cargo.toml            # Project configuration
├── .gitignore            # Git ignore rules
└── README.md             # This file
```

## 🔧 Dependencies

- **serde** - Serialization framework
- **serde_json** - JSON serialization
- **thiserror** - Error handling utilities
- **sha1** - SHA-1 hashing
- **reqwest** - HTTP client for tracker communication
- **urlencoding** - URL parameter encoding
- **url** - URL parsing and manipulation
- **bytes** - Byte buffer utilities

## 📚 Implementation Notes

### Bencode Format

The bencode parser handles the complete BitTorrent data format:

- Strings: `5:hello` → "hello"
- Integers: `i42e` → 42
- Lists: `l5:hello3:worlde` → ["hello", "world"]
- Dictionaries: `d3:key5:valuee` → {"key": "value"}

### Torrent File Structure

Supports both single-file and multi-file torrents:

- **Single-file**: Contains `length` field
- **Multi-file**: Contains `files` array with individual file entries

### Tracker Protocol

Implements HTTP tracker announce requests with required parameters:

- `info_hash` - SHA-1 hash of torrent info section
- `peer_id` - 20-byte unique peer identifier
- `port` - Listening port number
- `uploaded`/`downloaded`/`left` - Transfer statistics

### Peer Protocol

Basic implementation of BitTorrent peer handshake:

- Protocol identifier: "BitTorrent protocol"
- 8 reserved bytes (all zeros)
- 20-byte info hash
- 20-byte peer ID

## 🎯 Next Steps

To complete the BitTorrent client, the following features need implementation:

1. **Piece Management**

   - Piece selection algorithms (rarest-first, random)
   - Piece verification using SHA-1 hashes
   - Piece request/response handling

2. **Peer Communication**

   - Complete message protocol (choke, unchoke, interested, not interested)
   - Bitfield message handling
   - Request/have/piece message exchange

3. **File System**

   - Piece-to-file mapping
   - File writing and verification
   - Directory structure creation

4. **Download Logic**
   - Download progress tracking
   - Connection management for multiple peers
   - Choking/unchoking strategies

## 🤝 Contributing

This is an educational project. Contributions, suggestions, and improvements are welcome!

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/bittorrent-rust.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes and test them: `make quick`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to your branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration:

- **Tests**: Runs on multiple Rust versions (stable, beta, nightly)
- **Builds**: Tests on Linux, Windows, and macOS
- **Security**: Automated security audits with `cargo audit`
- **Dependencies**: Automated dependency updates with Dependabot
- **Documentation**: Automatic documentation generation
- **Coverage**: Code coverage reporting (on main branch)

### Code Quality

Before submitting a PR, ensure:

- `make quick` passes (format + lint + test)
- All tests pass: `make test`
- Code is properly formatted: `make fmt`
- No clippy warnings: `make clippy`
- Documentation is updated if needed

## 📄 License

This project is for educational purposes. Please respect copyright laws and only download content you have permission to access.

## ⚠️ Disclaimer

This software is provided for educational purposes only. Users are responsible for complying with applicable laws and respecting copyright when using BitTorrent technology.
