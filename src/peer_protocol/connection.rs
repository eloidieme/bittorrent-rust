use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::peer_protocol::error::{PeerProtocolError, Result};

/// Represents a connection to a BitTorrent peer
pub struct PeerConnection {
    stream: TcpStream,
    peer_addr: SocketAddr,
    peer_id: Option<String>,
    info_hash: Vec<u8>,
}

impl PeerConnection {
    /// Creates a new peer connection by connecting to the specified address
    pub fn connect<A: ToSocketAddrs>(addr: A, info_hash: &[u8], peer_id: &str) -> Result<Self> {
        let peer_addr = addr
            .to_socket_addrs()
            .map_err(|e| PeerProtocolError::InvalidPeerAddress(e.to_string()))?
            .next()
            .ok_or_else(|| {
                PeerProtocolError::InvalidPeerAddress("No valid address found".to_string())
            })?;

        let stream = TcpStream::connect(peer_addr)
            .map_err(|e| PeerProtocolError::ConnectionFailed(e.to_string()))?;

        // Set timeouts for the connection
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        stream.set_write_timeout(Some(Duration::from_secs(30)))?;

        let mut connection = Self {
            stream,
            peer_addr,
            peer_id: Some(peer_id.to_string()),
            info_hash: info_hash.to_vec(),
        };

        // Perform handshake
        connection.perform_handshake()?;

        Ok(connection)
    }

    /// Performs the BitTorrent handshake with the peer
    fn perform_handshake(&mut self) -> Result<()> {
        let handshake_message = self.build_handshake_message();

        // Send handshake
        self.stream.write_all(&handshake_message)?;
        self.stream.flush()?;

        // Read handshake response
        let mut response = vec![0u8; 68];
        let mut bytes_read = 0;

        while bytes_read < 68 {
            let n = self.stream.read(&mut response[bytes_read..])?;
            if n == 0 {
                return Err(PeerProtocolError::ConnectionClosed);
            }
            bytes_read += n;
        }

        // Validate handshake response
        self.validate_handshake_response(&response)?;

        Ok(())
    }

    /// Builds the handshake message according to BitTorrent protocol
    fn build_handshake_message(&self) -> Vec<u8> {
        let mut handshake: Vec<u8> = vec![19]; // Protocol string length
        handshake.extend_from_slice("BitTorrent protocol".as_bytes());
        handshake.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // Reserved bytes
        handshake.extend_from_slice(&self.info_hash);
        handshake.extend_from_slice(self.peer_id.as_ref().unwrap().as_bytes());
        handshake
    }

    /// Validates the handshake response from the peer
    fn validate_handshake_response(&mut self, response: &[u8]) -> Result<()> {
        if response.len() != 68 {
            return Err(PeerProtocolError::InvalidHandshakeLength {
                expected: 68,
                actual: response.len(),
            });
        }

        // Check protocol string
        if response[0] != 19 || &response[1..20] != "BitTorrent protocol".as_bytes() {
            return Err(PeerProtocolError::InvalidProtocolIdentifier);
        }

        // Check info hash
        if response[28..48] != self.info_hash {
            return Err(PeerProtocolError::InfoHashMismatch);
        }

        // Store peer's ID
        self.peer_id = Some(String::from_utf8_lossy(&response[48..68]).to_string());

        Ok(())
    }

    /// Sends a message to the peer and returns the response
    pub fn send_message(&mut self, message: &[u8]) -> Result<Vec<u8>> {
        self.stream.write_all(message)?;
        self.stream.flush()?;

        // Read response (assuming 68 bytes for now, but this should be made more flexible)
        let mut response = vec![0u8; 68];
        let mut bytes_read = 0;

        while bytes_read < 68 {
            let n = self.stream.read(&mut response[bytes_read..])?;
            if n == 0 {
                return Err(PeerProtocolError::ConnectionClosed);
            }
            bytes_read += n;
        }

        response.truncate(bytes_read);
        Ok(response)
    }

    /// Sends a message without expecting a response
    pub fn send_message_no_response(&mut self, message: &[u8]) -> Result<()> {
        self.stream.write_all(message)?;
        self.stream.flush()?;
        Ok(())
    }

    /// Reads a message from the peer
    pub fn read_message(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let bytes_read = self.stream.read(buffer)?;
        if bytes_read == 0 {
            return Err(PeerProtocolError::ConnectionClosed);
        }
        Ok(bytes_read)
    }

    /// Gets the peer's address
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// Gets the peer's ID (if available)
    pub fn peer_id(&self) -> Option<&str> {
        self.peer_id.as_deref()
    }

    /// Gets the info hash
    pub fn info_hash(&self) -> &[u8] {
        &self.info_hash
    }

    /// Checks if the connection is still alive
    pub fn is_connected(&self) -> bool {
        // This is a simple check - in a real implementation you might want to send a ping
        true
    }
}

impl Drop for PeerConnection {
    fn drop(&mut self) {
        // Connection will be closed automatically when TcpStream is dropped
    }
}
