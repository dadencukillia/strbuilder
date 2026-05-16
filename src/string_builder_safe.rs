// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

use std::rc::Rc;

const STRING_CHUNK_BYTES_LEN: usize = 64 - size_of::<Option<Rc<StringChunk>>>();

#[repr(align(64))]
struct StringChunk {
    bytes: [u8; STRING_CHUNK_BYTES_LEN],
    prev: Option<Rc<StringChunk>>
}

pub struct StringBuilder {
    last_chunk: Option<Rc<StringChunk>>,
    bytes_count: usize,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self {
            last_chunk: None,
            bytes_count: 0,
        }
    }

    pub fn from(string: &str) -> Self {
        let string_bytes = string.as_bytes();
        let bytes_count = string_bytes.len();
        let chunks_count = (bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN;

        let mut last_chunk: Option<Rc<StringChunk>> = None;

        // Loop for splitting our string into chunks
        for chunk_index in 0..chunks_count {
            let current_chunk_start = chunk_index * STRING_CHUNK_BYTES_LEN;
            let current_chunk_end = (current_chunk_start + STRING_CHUNK_BYTES_LEN - 1).min(bytes_count - 1);
            let mut current_string_chunk = StringChunk {
                bytes: [0; STRING_CHUNK_BYTES_LEN],
                prev: last_chunk.clone()
            };

            for chunk_byte_index in 0..=(current_chunk_end - current_chunk_start) {
                current_string_chunk.bytes[chunk_byte_index] = string_bytes[chunk_byte_index + current_chunk_start];
            }

            last_chunk = Some(Rc::new(current_string_chunk))
        }

        Self { 
            last_chunk,
            bytes_count,
        }
    }

    pub fn push_str(&mut self, string: &str) {
        if string.is_empty() { return; }

        let mut chunks = (self.bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN;

        if chunks == 0 {
            self.last_chunk = Some(Rc::new(StringChunk {
                bytes: [0; STRING_CHUNK_BYTES_LEN],
                prev: None
            }));

            chunks = 1;
        }

        let mut chunk_left_size = chunks * STRING_CHUNK_BYTES_LEN - self.bytes_count;
        let mut bytes_iter = string.bytes();

        while let Some(string_byte) = bytes_iter.next() {
            if chunk_left_size == 0 {
                chunk_left_size = STRING_CHUNK_BYTES_LEN;
                self.last_chunk = Some(Rc::new(StringChunk { 
                    bytes: [0; STRING_CHUNK_BYTES_LEN],
                    prev: self.last_chunk.clone()
                }));
            }

            if let Some(last_chunk) = Rc::get_mut(self.last_chunk.as_mut().unwrap()) {
                last_chunk.bytes[STRING_CHUNK_BYTES_LEN - chunk_left_size] = string_byte;
            }

            chunk_left_size -= 1;
        }

        self.bytes_count += string.len();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0; self.bytes_count];

        let chunks = (self.bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN;
        let mut remaining_chunk_size = if chunks * STRING_CHUNK_BYTES_LEN == self.bytes_count { 
            STRING_CHUNK_BYTES_LEN 
        } else { 
            self.bytes_count - (chunks - 1) * STRING_CHUNK_BYTES_LEN 
        };
        let mut current_chunk = self.last_chunk.clone();
        let mut index = self.bytes_count;

        while let Some(ref current_chunk_some) = current_chunk {
            bytes[index - 1] = current_chunk_some.bytes[remaining_chunk_size - 1];

            remaining_chunk_size -= 1;
            index -= 1;

            if remaining_chunk_size == 0 {
                current_chunk = current_chunk_some.prev.clone();
                remaining_chunk_size = STRING_CHUNK_BYTES_LEN;
            }
        }

        bytes
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.to_bytes()).into()
    }
}
