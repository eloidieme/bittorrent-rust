use super::*;
use std::collections::HashMap;

#[cfg(test)]
mod basic_parsing {
    use super::*;

    #[test]
    fn parse_string() {
        let (value, rest) = parse_bencoded_value(b"5:hello").unwrap();
        assert_eq!(value, Value::String(b"hello"));
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_empty_string() {
        let (value, rest) = parse_bencoded_value(b"0:").unwrap();
        assert_eq!(value, Value::String(b""));
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_integer() {
        let (value, rest) = parse_bencoded_value(b"i42e").unwrap();
        assert_eq!(value, Value::Integer(42));
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_negative_integer() {
        let (value, rest) = parse_bencoded_value(b"i-42e").unwrap();
        assert_eq!(value, Value::Integer(-42));
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_zero() {
        let (value, rest) = parse_bencoded_value(b"i0e").unwrap();
        assert_eq!(value, Value::Integer(0));
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_list() {
        let (value, rest) = parse_bencoded_value(b"li42e5:helloe").unwrap();
        match value {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], Value::Integer(42));
                assert_eq!(items[1], Value::String(b"hello"));
            }
            _ => panic!("Expected list"),
        }
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_empty_list() {
        let (value, rest) = parse_bencoded_value(b"le").unwrap();
        match value {
            Value::List(items) => assert_eq!(items.len(), 0),
            _ => panic!("Expected list"),
        }
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_dict() {
        let (value, rest) = parse_bencoded_value(b"d3:key5:valuee").unwrap();
        match value {
            Value::Dict(map) => {
                assert_eq!(map.len(), 1);
                assert_eq!(map.get(b"key".as_slice()), Some(&Value::String(b"value")));
            }
            _ => panic!("Expected dict"),
        }
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_empty_dict() {
        let (value, rest) = parse_bencoded_value(b"de").unwrap();
        match value {
            Value::Dict(map) => assert_eq!(map.len(), 0),
            _ => panic!("Expected dict"),
        }
        assert_eq!(rest, b"");
    }

    #[test]
    fn parse_nested_structures() {
        let data = b"d4:listli1ei2ee3:str5:helloe";
        let (value, rest) = parse_bencoded_value(data).unwrap();
        match value {
            Value::Dict(map) => {
                assert_eq!(map.len(), 2);
                assert_eq!(map.get(b"str".as_slice()), Some(&Value::String(b"hello")));
                if let Some(Value::List(items)) = map.get(b"list".as_slice()) {
                    assert_eq!(items.len(), 2);
                    assert_eq!(items[0], Value::Integer(1));
                    assert_eq!(items[1], Value::Integer(2));
                } else {
                    panic!("Expected list in dict");
                }
            }
            _ => panic!("Expected dict"),
        }
        assert_eq!(rest, b"");
    }
}

#[cfg(test)]
mod error_handling {
    use super::*;

    #[test]
    fn unexpected_eof_string() {
        let result = parse_bencoded_value(b"5:hel");
        assert!(matches!(result, Err(Error::InsufficientStringBytes { .. })));
    }

    #[test]
    fn unexpected_eof_integer() {
        let result = parse_bencoded_value(b"i42");
        assert!(matches!(result, Err(Error::MissingTerminator { .. })));
    }

    #[test]
    fn unexpected_eof_list() {
        let result = parse_bencoded_value(b"li42e");
        assert!(matches!(result, Err(Error::MissingTerminator { .. })));
    }

    #[test]
    fn unexpected_eof_dict() {
        let result = parse_bencoded_value(b"d3:key");
        assert!(matches!(result, Err(Error::UnexpectedEof { .. })));
    }

    #[test]
    fn invalid_string_length() {
        let result = parse_bencoded_value(b"abc:hello");
        assert!(matches!(result, Err(Error::UnexpectedByte { .. })));
    }

    #[test]
    fn invalid_integer_syntax_empty() {
        let result = parse_bencoded_value(b"ie");
        assert!(matches!(
            result,
            Err(Error::InvalidIntegerSyntax {
                reason: "empty",
                ..
            })
        ));
    }

    #[test]
    fn invalid_integer_syntax_negative_zero() {
        let result = parse_bencoded_value(b"i-0e");
        assert!(matches!(
            result,
            Err(Error::InvalidIntegerSyntax {
                reason: "negative zero",
                ..
            })
        ));
    }

    #[test]
    fn invalid_integer_syntax_leading_zero() {
        let result = parse_bencoded_value(b"i01e");
        assert!(matches!(
            result,
            Err(Error::InvalidIntegerSyntax {
                reason: "leading zero",
                ..
            })
        ));
    }

    #[test]
    fn invalid_integer_syntax_leading_zero_negative() {
        let result = parse_bencoded_value(b"i-01e");
        assert!(matches!(
            result,
            Err(Error::InvalidIntegerSyntax {
                reason: "leading zero after '-'",
                ..
            })
        ));
    }

    #[test]
    fn invalid_integer_syntax_non_digit() {
        let result = parse_bencoded_value(b"i12ae");
        assert!(matches!(
            result,
            Err(Error::InvalidIntegerSyntax {
                reason: "non-digit",
                ..
            })
        ));
    }

    #[test]
    fn non_string_dict_key() {
        let result = parse_bencoded_value(b"di42e5:valuee");
        assert!(matches!(result, Err(Error::UnexpectedByte { .. })));
    }

    #[test]
    fn depth_limit_exceeded() {
        // Create a deeply nested structure that exceeds MAX_DEPTH
        let mut data = Vec::new();
        for _ in 0..1025 {
            data.push(b'l');
        }
        for _ in 0..1025 {
            data.push(b'e');
        }

        let result = parse_bencoded_value(&data);
        println!("{:?}", result);
        assert!(matches!(result, Err(Error::DepthLimit { .. })));
    }
}

#[cfg(test)]
mod overflow_tests {
    use super::*;

    #[test]
    fn string_length_overflow() {
        // Create a string length that would overflow usize
        let data = b"18446744073709551616:hello";
        let result = parse_bencoded_value(data);
        assert!(matches!(result, Err(Error::InvalidStringLength { .. })));
    }

    #[test]
    fn integer_overflow() {
        // Test integer overflow
        let data = b"i9223372036854775808e"; // i64::MAX + 1
        let result = parse_bencoded_value(data);
        assert!(matches!(result, Err(Error::InvalidIntegerValue { .. })));
    }

    #[test]
    fn integer_underflow() {
        // Test integer underflow
        let data = b"i-9223372036854775809e"; // i64::MIN - 1
        let result = parse_bencoded_value(data);
        assert!(matches!(result, Err(Error::InvalidIntegerValue { .. })));
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn parse_bencoded_all_success() {
        let result = parse_bencoded_all(b"i42e");
        assert!(matches!(result, Ok(Value::Integer(42))));
    }

    #[test]
    fn parse_bencoded_all_trailing_bytes() {
        let result = parse_bencoded_all(b"i42ehello");
        assert!(matches!(result, Err(Error::UnexpectedByte { .. })));
    }

    #[test]
    fn value_as_bytes() {
        let value = Value::String(b"hello");
        assert_eq!(value.as_bytes(), Some(b"hello".as_slice()));
        assert_eq!(Value::Integer(42).as_bytes(), None);
    }

    #[test]
    fn value_as_int() {
        let value = Value::Integer(42);
        assert_eq!(value.as_int(), Some(42));
        assert_eq!(Value::String(b"hello").as_int(), None);
    }

    #[test]
    fn value_get_dict() {
        let mut map = HashMap::new();
        map.insert(b"key".as_slice(), Value::String(b"value"));
        let value = Value::Dict(map);

        assert_eq!(value.get(b"key"), Some(&Value::String(b"value")));
        assert_eq!(value.get(b"missing"), None);
        assert_eq!(Value::String(b"hello").get(b"key"), None);
    }

    #[test]
    fn extract_dict_value_range_success() {
        let data = b"d3:key5:valuee";
        let mut cursor = Cursor::new(data);
        let range = extract_dict_value_range(&mut cursor, "key").unwrap();
        assert_eq!(&data[range], b"5:value");
    }

    #[test]
    fn extract_dict_value_range_key_not_found() {
        let data = b"d3:key5:valuee";
        let mut cursor = Cursor::new(data);
        let result = extract_dict_value_range(&mut cursor, "missing");
        assert!(matches!(result, Err(Error::DictKeyNotFound { .. })));
    }

    #[test]
    fn extract_dict_value_range_non_string_key() {
        let data = b"di42e5:valuee";
        let mut cursor = Cursor::new(data);
        let result = extract_dict_value_range(&mut cursor, "key");
        assert!(matches!(result, Err(Error::UnexpectedByte { .. })));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn complex_torrent_like_structure() {
        let torrent_data = include_bytes!("../../debian.iso.torrent");
        let (bencode_value, rest) = parse_bencoded_value(torrent_data).unwrap();
        assert_eq!(rest, b""); // Should consume all data

        let Value::Dict(root_dict) = bencode_value else {
            panic!("Expected root to be a dictionary");
        };

        assert!(root_dict.contains_key(b"announce".as_slice()));
        assert!(root_dict.contains_key(b"info".as_slice()));

        let Some(Value::String(announce)) = root_dict.get(b"announce".as_slice()) else {
            panic!("Expected 'announce' to be a string");
        };
        assert!(announce.starts_with(b"http"));

        let Some(Value::Dict(info_dict)) = root_dict.get(b"info".as_slice()) else {
            panic!("Expected 'info' to be a dictionary");
        };

        assert!(info_dict.contains_key(b"name".as_slice()));
        assert!(info_dict.contains_key(b"piece length".as_slice()));
        assert!(info_dict.contains_key(b"pieces".as_slice()));

        let Some(Value::String(name)) = info_dict.get(b"name".as_slice()) else {
            panic!("Expected 'name' to be a string");
        };
        assert!(!name.is_empty());

        let Some(Value::Integer(piece_length)) = info_dict.get(b"piece length".as_slice()) else {
            panic!("Expected 'piece length' to be an integer");
        };
        assert!(*piece_length > 0);

        let Some(Value::String(pieces)) = info_dict.get(b"pieces".as_slice()) else {
            panic!("Expected 'pieces' to be a string");
        };
        assert!(!pieces.is_empty());
        assert_eq!(
            pieces.len() % 20,
            0,
            "Pieces should be multiple of 20 bytes (SHA-1 hashes)"
        );

        let has_length = info_dict.contains_key(b"length".as_slice());
        let has_files = info_dict.contains_key(b"files".as_slice());

        if has_length {
            let Some(Value::Integer(length)) = info_dict.get(b"length".as_slice()) else {
                panic!("Expected 'length' to be an integer for single file torrent");
            };
            assert!(*length > 0);
            assert!(
                !has_files,
                "Single file torrent should not have 'files' key"
            );
        } else if has_files {
            let Some(Value::List(files)) = info_dict.get(b"files".as_slice()) else {
                panic!("Expected 'files' to be a list for multi-file torrent");
            };
            assert!(!files.is_empty(), "Files list should not be empty");

            for (i, file) in files.iter().enumerate() {
                let Value::Dict(file_dict) = file else {
                    panic!("Expected file[{}] to be a dictionary", i);
                };
                assert!(
                    file_dict.contains_key(b"length".as_slice()),
                    "File[{}] missing 'length'",
                    i
                );
                assert!(
                    file_dict.contains_key(b"path".as_slice()),
                    "File[{}] missing 'path'",
                    i
                );

                let Some(Value::Integer(file_length)) = file_dict.get(b"length".as_slice()) else {
                    panic!("Expected file[{}].length to be an integer", i);
                };
                assert!(*file_length > 0, "File[{}] length should be positive", i);

                let Some(Value::List(path_list)) = file_dict.get(b"path".as_slice()) else {
                    panic!("Expected file[{}].path to be a list", i);
                };
                assert!(
                    !path_list.is_empty(),
                    "File[{}] path should not be empty",
                    i
                );

                for (j, path_component) in path_list.iter().enumerate() {
                    let Value::String(_) = path_component else {
                        panic!("Expected file[{}].path[{}] to be a string", i, j);
                    };
                }
            }
        } else {
            panic!("Torrent must have either 'length' (single file) or 'files' (multi-file)");
        }

        if let Some(Value::List(announce_list)) = root_dict.get(b"announce-list".as_slice()) {
            for (i, tier) in announce_list.iter().enumerate() {
                let Value::List(tier_list) = tier else {
                    panic!("Expected announce-list[{}] to be a list", i);
                };
                for (j, announce_url) in tier_list.iter().enumerate() {
                    let Value::String(url) = announce_url else {
                        panic!("Expected announce-list[{}][{}] to be a string", i, j);
                    };
                    assert!(
                        url.starts_with(b"http"),
                        "Announce URL should start with http"
                    );
                }
            }
        }

        if let Some(Value::Integer(creation_date)) = root_dict.get(b"creation date".as_slice()) {
            assert!(*creation_date > 0, "Creation date should be positive");
        }

        if let Some(Value::String(_comment)) = root_dict.get(b"comment".as_slice()) {
            // Comment can be empty, just verify it's a string
        }

        if let Some(Value::String(_created_by)) = root_dict.get(b"created by".as_slice()) {
            // Created by can be empty, just verify it's a string
        }

        if let Some(Value::String(_encoding)) = root_dict.get(b"encoding".as_slice()) {
            // Encoding should be a valid string (usually "UTF-8")
        }
    }

    #[test]
    fn multiple_values_with_rest() {
        let data = b"i42e5:hello";
        let (value1, rest1) = parse_bencoded_value(data).unwrap();
        assert_eq!(value1, Value::Integer(42));
        assert_eq!(rest1, b"5:hello");

        let (value2, rest2) = parse_bencoded_value(rest1).unwrap();
        assert_eq!(value2, Value::String(b"hello"));
        assert_eq!(rest2, b"");
    }

    #[test]
    fn large_string() {
        let large_string = "x".repeat(10000);
        let data = format!("{}:{}", large_string.len(), large_string);
        let (value, rest) = parse_bencoded_value(data.as_bytes()).unwrap();
        assert_eq!(value, Value::String(large_string.as_bytes()));
        assert_eq!(rest, b"");
    }

    #[test]
    fn large_list() {
        let mut data = Vec::new();
        data.push(b'l');
        for i in 0..1000 {
            data.extend_from_slice(format!("i{}e", i).as_bytes());
        }
        data.push(b'e');

        let (value, rest) = parse_bencoded_value(&data).unwrap();
        assert_eq!(rest, b"");

        match value {
            Value::List(items) => {
                assert_eq!(items.len(), 1000);
                for (i, item) in items.iter().enumerate() {
                    assert_eq!(item, &Value::Integer(i as i64));
                }
            }
            _ => panic!("Expected list"),
        }
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn empty_input() {
        let result = parse_bencoded_value(b"");
        assert!(matches!(result, Err(Error::UnexpectedEof { .. })));
    }

    #[test]
    fn invalid_first_byte() {
        let result = parse_bencoded_value(b"x");
        assert!(matches!(result, Err(Error::UnexpectedByte { .. })));
    }

    #[test]
    fn string_with_zero_length() {
        let (value, rest) = parse_bencoded_value(b"0:").unwrap();
        assert_eq!(value, Value::String(b""));
        assert_eq!(rest, b"");
    }

    #[test]
    fn integer_min_max() {
        let (value, rest) = parse_bencoded_value(b"i9223372036854775807e").unwrap();
        assert_eq!(value, Value::Integer(i64::MAX));
        assert_eq!(rest, b"");

        let (value, rest) = parse_bencoded_value(b"i-9223372036854775808e").unwrap();
        assert_eq!(value, Value::Integer(i64::MIN));
        assert_eq!(rest, b"");
    }

    #[test]
    fn dict_with_duplicate_keys() {
        // Bencode allows duplicate keys, last one wins
        let data = b"d3:key5:first3:key6:seconde";
        let (value, rest) = parse_bencoded_value(data).unwrap();
        match value {
            Value::Dict(map) => {
                assert_eq!(map.get(b"key".as_slice()), Some(&Value::String(b"second")));
            }
            _ => panic!("Expected dict"),
        }
        assert_eq!(rest, b"");
    }
}
