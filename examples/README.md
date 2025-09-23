# Examples

This directory contains example applications demonstrating how to use the BitTorrent Rust library.

## Basic Client

The `basic-client` example shows the fundamental BitTorrent workflow:

1. Parse a torrent file
2. Extract metainfo and compute info hash
3. Contact the tracker to discover peers
4. Connect to a peer and perform handshake

### Running the Example

```bash
cd examples/basic-client
cargo run -- ../../examples/torrents/debian.iso.torrent
```

### What it does

- Loads and parses a .torrent file
- Generates a unique peer ID
- Contacts the BitTorrent tracker
- Connects to the first available peer
- Performs the BitTorrent handshake
- Sends a keep-alive message

This demonstrates the basic building blocks of a BitTorrent client, though it doesn't actually download any files yet.

## Torrent Files

The `torrents/` directory contains sample .torrent files for testing:

- `debian.iso.torrent` - Debian ISO torrent (large file, good for testing)

## Debug Files

The `debug/` directory contains temporary files generated during development and testing:

- `parsed.json` - Parsed torrent file data (for debugging)
- `peers.json` - Peer list from tracker response (for debugging)

These files are automatically generated and can be safely deleted.
