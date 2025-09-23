#[cfg(test)]
mod tests {
    use crate::bencode::Value;
    use std::collections::HashMap;

    mod error_tests {
        use crate::announce::error::Error;

        #[test]
        fn test_error_display() {
            let error = Error::InvalidInfoHash(15);
            assert!(
                error
                    .to_string()
                    .contains("invalid info_hash length 15, expected 20")
            );

            let error = Error::InvalidPeerId(10);
            assert!(
                error
                    .to_string()
                    .contains("invalid peer_id length 10, expected 20")
            );

            let error = Error::Status {
                status: reqwest::StatusCode::NOT_FOUND,
                snippet: "Not found".to_string(),
            };
            assert!(
                error
                    .to_string()
                    .contains("tracker returned non-success status 404")
            );
        }

        #[test]
        fn test_error_from_url_parse_error() {
            let url_error = url::ParseError::EmptyHost;
            let announce_error: Error = url_error.into();
            assert!(matches!(announce_error, Error::Url(_)));
        }

        #[test]
        fn test_error_from_reqwest_error() {
            // Create a reqwest error by trying to make a request with an invalid URL
            let client = reqwest::blocking::Client::new();
            let reqwest_error = client.get("not-a-url").send().unwrap_err();
            let announce_error: Error = reqwest_error.into();
            assert!(matches!(announce_error, Error::Http(_)));
        }
    }

    mod params_tests {
        use crate::announce::params::{AnnounceEvent, AnnounceParams};

        #[test]
        fn test_announce_event_as_str() {
            assert_eq!(AnnounceEvent::Started.as_str(), "started");
            assert_eq!(AnnounceEvent::Stopped.as_str(), "stopped");
            assert_eq!(AnnounceEvent::Completed.as_str(), "completed");
            assert_eq!(AnnounceEvent::Empty.as_str(), "empty");
        }

        #[test]
        fn test_announce_params_new_success() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];

            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000);
            assert!(params.is_ok());

            let params = params.unwrap();
            assert_eq!(params.info_hash, &info_hash);
            assert_eq!(params.peer_id, &peer_id);
            assert_eq!(params.port, 6881);
            assert_eq!(params.uploaded, 0);
            assert_eq!(params.downloaded, 0);
            assert_eq!(params.left, 1000);
            assert!(params.compact);
            assert_eq!(params.numwant, None);
            assert_eq!(params.event, None);
            assert_eq!(params.key, None);
            assert_eq!(params.ip, None);
        }

        #[test]
        fn test_announce_params_new_invalid_info_hash() {
            let info_hash = [1u8; 15]; // Too short
            let peer_id = [2u8; 20];

            let result = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000);
            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                crate::announce::error::Error::InvalidInfoHash(15)
            ));
        }

        #[test]
        fn test_announce_params_new_invalid_peer_id() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 10]; // Too short

            let result = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000);
            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                crate::announce::error::Error::InvalidPeerId(10)
            ));
        }

        #[test]
        fn test_announce_params_validate() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000).unwrap();

            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_announce_params_validate_invalid() {
            let params = AnnounceParams {
                info_hash: &[1u8; 15], // Invalid length
                peer_id: &[2u8; 20],
                port: 6881,
                uploaded: 0,
                downloaded: 0,
                left: 1000,
                compact: true,
                numwant: None,
                event: None,
                key: None,
                ip: None,
            };

            assert!(params.validate().is_err());
        }

        #[test]
        fn test_build_query_string_basic() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 100, 200, 1000).unwrap();

            let query = params.build_query_string();
            assert!(query.contains("info_hash="));
            assert!(query.contains("peer_id="));
            assert!(query.contains("port=6881"));
            assert!(query.contains("uploaded=100"));
            assert!(query.contains("downloaded=200"));
            assert!(query.contains("left=1000"));
            assert!(query.contains("compact=1"));
        }

        #[test]
        fn test_build_query_string_with_options() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000)
                .unwrap()
                .with_event(AnnounceEvent::Started)
                .with_numwant(50)
                .with_key("test-key")
                .with_ip("192.168.1.1")
                .with_compact(false);

            let query = params.build_query_string();
            assert!(query.contains("event=started"));
            assert!(query.contains("numwant=50"));
            assert!(query.contains("key=test-key"));
            assert!(query.contains("ip=192.168.1.1"));
            assert!(!query.contains("compact=1"));
        }

        #[test]
        fn test_with_event() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000)
                .unwrap()
                .with_event(AnnounceEvent::Completed);

            assert_eq!(params.event, Some(AnnounceEvent::Completed));
        }

        #[test]
        fn test_with_numwant() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000)
                .unwrap()
                .with_numwant(100);

            assert_eq!(params.numwant, Some(100));
        }

        #[test]
        fn test_with_key() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000)
                .unwrap()
                .with_key("my-key");

            assert_eq!(params.key, Some("my-key"));
        }

        #[test]
        fn test_with_ip() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000)
                .unwrap()
                .with_ip("10.0.0.1");

            assert_eq!(params.ip, Some("10.0.0.1"));
        }

        #[test]
        fn test_with_compact() {
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000)
                .unwrap()
                .with_compact(false);

            assert!(!params.compact);
        }
    }

    mod response_tests {
        use super::*;
        use crate::announce::response::{Peer, PeersList};

        #[test]
        fn test_peer_creation() {
            let peer = Peer {
                ip: "192.168.1.1".to_string(),
                port: 6881,
            };

            assert_eq!(peer.ip, "192.168.1.1");
            assert_eq!(peer.port, 6881);
        }

        #[test]
        fn test_peers_list_creation() {
            let peers = vec![
                Peer {
                    ip: "192.168.1.1".to_string(),
                    port: 6881,
                },
                Peer {
                    ip: "192.168.1.2".to_string(),
                    port: 6882,
                },
            ];

            let peers_list = PeersList {
                interval: 1800,
                peers,
            };

            assert_eq!(peers_list.interval, 1800);
            assert_eq!(peers_list.peers.len(), 2);
            assert_eq!(peers_list.peers[0].ip, "192.168.1.1");
            assert_eq!(peers_list.peers[1].port, 6882);
        }

        #[test]
        fn test_peers_list_from_bencode() {
            // Create a bencoded response similar to what a tracker would return
            let mut peers_list = Vec::new();
            peers_list.push(Value::Dict({
                let mut peer = HashMap::new();
                peer.insert(b"ip".as_slice(), Value::String(b"192.168.1.1".as_slice()));
                peer.insert(b"port".as_slice(), Value::Integer(6881));
                peer
            }));
            peers_list.push(Value::Dict({
                let mut peer = HashMap::new();
                peer.insert(b"ip".as_slice(), Value::String(b"192.168.1.2".as_slice()));
                peer.insert(b"port".as_slice(), Value::Integer(6882));
                peer
            }));

            let mut root = HashMap::new();
            root.insert(b"interval".as_slice(), Value::Integer(1800));
            root.insert(b"peers".as_slice(), Value::List(peers_list));

            let response = Value::Dict(root);
            let peers_list = PeersList::from(response).unwrap();

            assert_eq!(peers_list.interval, 1800);
            assert_eq!(peers_list.peers.len(), 2);
            assert_eq!(peers_list.peers[0].ip, "192.168.1.1");
            assert_eq!(peers_list.peers[0].port, 6881);
            assert_eq!(peers_list.peers[1].ip, "192.168.1.2");
            assert_eq!(peers_list.peers[1].port, 6882);
        }

        #[test]
        fn test_peers_list_from_bencode_missing_interval() {
            let mut root = HashMap::new();
            root.insert(b"peers".as_slice(), Value::List(vec![]));

            let response = Value::Dict(root);
            let result = PeersList::from(response);
            assert!(result.is_err());
        }

        #[test]
        fn test_peers_list_from_bencode_missing_peers() {
            let mut root = HashMap::new();
            root.insert(b"interval".as_slice(), Value::Integer(1800));

            let response = Value::Dict(root);
            let result = PeersList::from(response);
            assert!(result.is_err());
        }

        #[test]
        fn test_peers_list_from_bencode_invalid_peer() {
            let mut peers_list = Vec::new();
            peers_list.push(Value::Dict({
                let mut peer = HashMap::new();
                peer.insert(b"ip".as_slice(), Value::String(b"192.168.1.1".as_slice()));
                // Missing port field
                peer
            }));

            let mut root = HashMap::new();
            root.insert(b"interval".as_slice(), Value::Integer(1800));
            root.insert(b"peers".as_slice(), Value::List(peers_list));

            let response = Value::Dict(root);
            let peers_list = PeersList::from(response).unwrap();

            // Invalid peers should be filtered out
            assert_eq!(peers_list.peers.len(), 0);
        }
    }

    mod client_tests {
        use crate::announce::client::new_client;

        #[test]
        fn test_new_client() {
            let client = new_client();
            assert!(client.is_ok());

            let client = client.unwrap();
            // Test that we can create a request (though we won't send it)
            let url = "http://example.com";
            let _request = client.get(url);
            // RequestBuilder doesn't have is_ok(), but if we got here without panic, it's working
        }
    }

    mod announce_tests {
        use crate::announce::client::new_client;
        use crate::announce::params::AnnounceParams;
        use crate::announce::perform_announce;

        #[test]
        fn test_perform_announce_invalid_url() {
            let client = new_client().unwrap();
            let info_hash = [1u8; 20];
            let peer_id = [2u8; 20];
            let params = AnnounceParams::new(&info_hash, &peer_id, 6881, 0, 0, 1000).unwrap();

            let result = perform_announce(&client, "not-a-valid-url", params);
            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                crate::announce::error::Error::Url(_)
            ));
        }

        #[test]
        fn test_perform_announce_invalid_params() {
            let client = new_client().unwrap();
            let params = AnnounceParams {
                info_hash: &[1u8; 15], // Invalid length
                peer_id: &[2u8; 20],
                port: 6881,
                uploaded: 0,
                downloaded: 0,
                left: 1000,
                compact: true,
                numwant: None,
                event: None,
                key: None,
                ip: None,
            };

            let result = perform_announce(&client, "http://example.com/announce", params);
            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                crate::announce::error::Error::InvalidInfoHash(15)
            ));
        }

        // Note: We don't test successful HTTP requests here as they would require
        // a real tracker or mock server, which is better suited for integration tests
    }
}
