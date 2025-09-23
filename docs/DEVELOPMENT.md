# Development Notes

This document contains development notes and implementation details for the BitTorrent Rust project.

## Implementation Progress

### Completed Features

1. **Bencode Parser** - Complete implementation of the bencode format used by BitTorrent
2. **Torrent File Parsing** - Support for both single-file and multi-file torrents
3. **Info Hash Computation** - SHA-1 hashing of torrent info sections
4. **Tracker Communication** - HTTP tracker announce requests and response parsing
5. **Basic Peer Protocol** - TCP connection and handshake implementation

### Current Workflow

The basic client currently follows this workflow:

1. Parse the .torrent file using the bencode parser
2. Extract metainfo (announce URL, file info, piece hashes)
3. Compute the info hash from the raw info dictionary
4. Generate a unique peer ID
5. Contact the tracker with announce parameters
6. Parse the tracker response to get peer list
7. Connect to the first peer and perform handshake
8. Send a keep-alive message

### Next Steps

To complete the BitTorrent client, the following components need implementation:

#### Piece Management

- Piece selection algorithms (rarest-first, random)
- Piece verification using SHA-1 hashes
- Piece request/response handling
- Piece-to-file mapping

#### Peer Communication

- Complete message protocol implementation
- Choke/unchoke logic
- Interested/not interested messages
- Bitfield message handling
- Request/have/piece message exchange

#### File System

- File writing and verification
- Directory structure creation
- Piece storage and retrieval

#### Download Logic

- Download progress tracking
- Multiple peer connection management
- Choking/unchoking strategies
- End-to-end download orchestration

## Architecture Decisions

### Error Handling

- Uses `thiserror` for structured error types
- Each module has its own error types
- Comprehensive error context with byte offsets

### Memory Management

- Zero-copy parsing where possible using lifetimes
- Borrowed data structures to avoid unnecessary allocations
- Careful lifetime management for bencode parsing

### Protocol Implementation

- Modular design separating concerns
- Clear separation between tracker and peer protocols
- Extensible message handling system

## Testing Strategy

- Unit tests for each module
- Integration tests for complete workflows
- Property-based testing for bencode parser
- Error condition testing

## Performance Considerations

- Zero-copy parsing for large torrent files
- Efficient peer connection management
- Minimal memory allocations during parsing
- Streaming support for large files
