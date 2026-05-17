// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

use std::{
    fmt::Display, rc::Rc
};

const STRING_CHUNK_BYTES_LEN: usize = 1024 - size_of::<Option<Rc<StringChunk>>>();

#[repr(align(64))]
struct StringChunk {
    bytes: [u8; STRING_CHUNK_BYTES_LEN],
    prev: Option<Rc<StringChunk>>
}

/// A StringBuilder that uses linked list to avoid reallocations
/// A safe variant
///
/// Example
/// ```Rust
/// let mut string_builder = StringBuilder::from("Hello,");
/// // or
/// let mut string_builder = StringBuilder::new();
///
/// string_builder.push_str(" ");
/// string_builder.push_str("world!");
///
/// let result = string_builder.to_string();
/// // or
/// println!("{:?}", string_builder);
/// ```
pub struct StringBuilder {
    last_chunk: Option<Rc<StringChunk>>,
    bytes_count: usize,
}

// Private methods
impl StringBuilder {
    /// Collects all data from chunks into one u8 buffer
    /// Very expensive operation
    fn write_to_slice(&self, buffer: &mut [u8]) {
        let chunks = self.get_chunks_count();
        let mut remaining_chunk_size = if chunks * STRING_CHUNK_BYTES_LEN == self.bytes_count { 
            STRING_CHUNK_BYTES_LEN 
        } else { 
            self.bytes_count - (chunks - 1) * STRING_CHUNK_BYTES_LEN 
        };
        let mut current_chunk = self.last_chunk.clone();
        let mut index = self.bytes_count;

        while let Some(ref current_chunk_some) = current_chunk {
            index -= remaining_chunk_size;
            for i in 0..remaining_chunk_size {
                buffer[index + i] = current_chunk_some.bytes[i];
            }

            current_chunk = current_chunk_some.prev.clone();
            remaining_chunk_size = STRING_CHUNK_BYTES_LEN;
        }
    }

    /// Calculates the chunks count (linked list elements) by the bytes count field
    #[inline]
    fn get_chunks_count(&self) -> usize {
        (self.bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN
    }
}

// Public methods
impl StringBuilder {
    /// Creates a new `StringBuilder` instance
    pub fn new() -> Self {
        Self {
            last_chunk: None,
            bytes_count: 0,
        }
    }

    /// Adds a string slice into the StringBuilder buffer
    /// As a result, it concatenates the argument to the previous added strings
    pub fn push_str(&mut self, string: &str) {
        if string.is_empty() { return; }

        let chunks = self.get_chunks_count();
        let mut chunk_left_size = chunks * STRING_CHUNK_BYTES_LEN - self.bytes_count;
        let mut bytes_iter = string.bytes();
        let mut bytes_left = string.len();

        while bytes_left != 0 {
            if chunk_left_size == 0 {
                // Here's Rc counter increases to 2, but
                // returns back to 1 when we assign a new value to
                // self.last_chunk

                chunk_left_size = STRING_CHUNK_BYTES_LEN;
                self.last_chunk = Some(Rc::new(StringChunk { 
                    bytes: [0; STRING_CHUNK_BYTES_LEN],
                    prev: self.last_chunk.clone()
                }));
            }

            let chunk_bytes_to_fill = chunk_left_size.min(bytes_left);

            if let Some(last_chunk) = Rc::get_mut(self.last_chunk.as_mut().unwrap()) {
                let start_index = STRING_CHUNK_BYTES_LEN - chunk_left_size;
                for i in 0..chunk_bytes_to_fill {
                    last_chunk.bytes[start_index + i] = bytes_iter.next().unwrap();
                }

                bytes_left -= chunk_bytes_to_fill;
                chunk_left_size -= chunk_bytes_to_fill;
            } else {
                unreachable!("Each Rc has only 1 owner");
            }
        }

        self.bytes_count += string.len();
    }
}

impl From<&str> for StringBuilder {
    fn from(value: &str) -> Self {
        let string_bytes = value.as_bytes();
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
}

impl Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = if self.bytes_count <= 4096 {
            let mut static_buf: [u8; _] = [0; 4096];
            self.write_to_slice(&mut static_buf);
            String::from_utf8_lossy(&static_buf[..self.bytes_count]).into()
        } else {
            let mut heap_buf: Vec<u8> = vec![0; self.bytes_count];
            self.write_to_slice(&mut heap_buf);
            String::from_utf8_lossy(&heap_buf).into()
        };

        f.write_str(&str)
    }
}

impl Into<String> for StringBuilder {
    #[inline]
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<String> for StringBuilder {
    #[inline]
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StringBuilder {
    fn drop(&mut self) {
        let mut last_chunk: Option<Rc<StringChunk>> = self.last_chunk.take();

        while let Some(last_chunk_some) = last_chunk.as_mut() {
            if let Some(last_chunk_access) = Rc::get_mut(last_chunk_some) {
                last_chunk = last_chunk_access.prev.take();
            } else {
                unreachable!("Each Rc has only 1 owner");
            }
        }
    }
}
