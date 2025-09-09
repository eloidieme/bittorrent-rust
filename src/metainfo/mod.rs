mod error;
mod helpers;

use serde::Serialize;

use crate::{
    bencode::Value,
    metainfo::{
        error::{Error, Result},
        helpers::{as_bytes, as_list, as_str, as_u64, dict, get, maybe, read_path},
    },
};

#[derive(Debug, Serialize)]
pub struct Metainfo<'l> {
    pub announce: &'l str,
    pub info: Info<'l>,
}

#[derive(Debug, Serialize)]
pub struct File<'l> {
    pub length: u64,
    pub path: Vec<&'l str>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum Info<'l> {
    SingleFile {
        name: &'l str,
        piece_length: u64,
        pieces: &'l [u8],
        length: u64,
    },
    MultiFile {
        name: &'l str,
        piece_length: u64,
        pieces: &'l [u8],
        files: Vec<File<'l>>,
    },
}

impl<'l> Metainfo<'l> {
    pub fn from(data: &'l Value<'l>) -> Result<Self> {
        let root = dict(data, "<root>")?;

        let announce = as_str(get(root, "announce")?, "announce")?;

        let info_v = get(root, "info")?;
        let info_d = dict(&info_v, "info")?;

        let name = as_str(get(info_d, "name")?, "name")?;

        let piece_length = as_u64(get(info_d, "piece length")?, "piece length")?;
        if piece_length == 0 {
            return Err(error::Error::PieceLengthZero { got: piece_length });
        }

        let pieces = as_bytes(get(info_d, "pieces")?, "pieces")?;
        if pieces.len() % 20 != 0 {
            return Err(error::Error::PiecesNonMultipleOf20 { len: pieces.len() });
        }

        let has_length = maybe(info_d, "length").is_some();
        let has_files = maybe(info_d, "files").is_some();

        let info = match (has_length, has_files) {
            (true, true) => return Err(Error::BothLengthAndFiles),
            (false, false) => return Err(Error::LengthOrFilesMissing),

            (true, false) => {
                let length = as_u64(get(info_d, "length")?, "length")?;
                Info::SingleFile {
                    name,
                    piece_length,
                    pieces,
                    length,
                }
            }

            (false, true) => {
                let files_v = get(info_d, "files")?;
                let files_l = as_list(files_v, "files")?;
                if files_l.is_empty() {
                    return Err(Error::FilesEmpty);
                }

                let mut files = Vec::with_capacity(files_l.len());
                for (idx, f) in files_l.iter().enumerate() {
                    let fd = dict(f, "files[*]")?;
                    let length = as_u64(get(fd, "length")?, "files[*].length")?;
                    let path = read_path(fd, idx)?;
                    files.push(File { length, path });
                }

                Info::MultiFile {
                    name,
                    piece_length,
                    pieces,
                    files,
                }
            }
        };

        Ok(Metainfo { announce, info })
    }
}
