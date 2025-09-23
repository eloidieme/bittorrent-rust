use crate::announce::error::{Error, Result};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    Empty,
}

impl AnnounceEvent {
    pub fn as_str(self) -> &'static str {
        match self {
            AnnounceEvent::Started => "started",
            AnnounceEvent::Stopped => "stopped",
            AnnounceEvent::Completed => "completed",
            AnnounceEvent::Empty => "empty",
        }
    }
}

#[derive(Debug, Default)]
pub struct AnnounceParams<'a> {
    pub info_hash: &'a [u8], // must be 20 bytes
    pub peer_id: &'a [u8],   // must be 20 bytes
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,

    pub compact: bool,
    pub numwant: Option<u16>,
    pub event: Option<AnnounceEvent>,
    pub key: Option<&'a str>,
    pub ip: Option<&'a str>,
}

impl<'a> AnnounceParams<'a> {
    /// Creates a new `AnnounceParams` with validation.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidInfoHash` if info_hash is not exactly 20 bytes.
    /// Returns `Error::InvalidPeerId` if peer_id is not exactly 20 bytes.
    pub fn new(
        info_hash: &'a [u8],
        peer_id: &'a [u8],
        port: u16,
        uploaded: u64,
        downloaded: u64,
        left: u64,
    ) -> Result<Self> {
        if info_hash.len() != 20 {
            return Err(Error::InvalidInfoHash(info_hash.len()));
        }
        if peer_id.len() != 20 {
            return Err(Error::InvalidPeerId(peer_id.len()));
        }

        Ok(Self {
            info_hash,
            peer_id,
            port,
            uploaded,
            downloaded,
            left,
            compact: true, // Default to compact format
            numwant: None,
            event: None,
            key: None,
            ip: None,
        })
    }

    /// Validates the current parameters.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidInfoHash` if info_hash is not exactly 20 bytes.
    /// Returns `Error::InvalidPeerId` if peer_id is not exactly 20 bytes.
    pub fn validate(&self) -> Result<()> {
        if self.info_hash.len() != 20 {
            return Err(Error::InvalidInfoHash(self.info_hash.len()));
        }
        if self.peer_id.len() != 20 {
            return Err(Error::InvalidPeerId(self.peer_id.len()));
        }
        Ok(())
    }

    /// Builds the query string for the announce request.
    /// This method efficiently constructs the URL query parameters.
    pub fn build_query_string(&self) -> String {
        let mut query_parts = Vec::with_capacity(8); // Pre-allocate for common parameters

        query_parts.push(format!(
            "info_hash={}",
            urlencoding::encode_binary(self.info_hash)
        ));
        query_parts.push(format!(
            "peer_id={}",
            urlencoding::encode_binary(self.peer_id)
        ));
        query_parts.push(format!("port={}", self.port));
        query_parts.push(format!("uploaded={}", self.uploaded));
        query_parts.push(format!("downloaded={}", self.downloaded));
        query_parts.push(format!("left={}", self.left));

        // Add optional parameters
        if self.compact {
            query_parts.push("compact=1".to_string());
        }

        if let Some(numwant) = self.numwant {
            query_parts.push(format!("numwant={}", numwant));
        }

        if let Some(event) = self.event {
            query_parts.push(format!("event={}", event.as_str()));
        }

        if let Some(key) = self.key {
            query_parts.push(format!("key={}", urlencoding::encode(key)));
        }

        if let Some(ip) = self.ip {
            query_parts.push(format!("ip={}", urlencoding::encode(ip)));
        }

        query_parts.join("&")
    }

    /// Sets the event for this announce.
    pub fn with_event(mut self, event: AnnounceEvent) -> Self {
        self.event = Some(event);
        self
    }

    /// Sets the number of peers to request.
    pub fn with_numwant(mut self, numwant: u16) -> Self {
        self.numwant = Some(numwant);
        self
    }

    /// Sets the key for this announce.
    pub fn with_key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    /// Sets the IP address for this announce.
    pub fn with_ip(mut self, ip: &'a str) -> Self {
        self.ip = Some(ip);
        self
    }

    /// Sets whether to use compact peer format.
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}
