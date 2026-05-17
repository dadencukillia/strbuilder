// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

use std::{
    alloc::{Layout, alloc, dealloc},
    fmt::Display
};

const STRING_CHUNK_BYTES_LEN: usize = 1024 - 8;

#[repr(align(64))]
struct StringChunk {
    bytes: [u8; STRING_CHUNK_BYTES_LEN],
    prev: *const StringChunk
}

/// A StringBuilder that uses linked list to avoid reallocations
/// An unsafe variant
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
    last_chunk: *mut StringChunk,
    bytes_count: usize,
}

// Private methods
impl StringBuilder {
    /// Allocates a place for a StringChunk and puts your argument there
    /// Returns a pointer to this place
    fn allocate_new_chunk(chunk: StringChunk) -> *mut StringChunk {
        let alloc_layout = Layout::new::<StringChunk>();
        unsafe {
            let allocated = alloc(alloc_layout) as *mut StringChunk;
            if allocated.is_null() {
                panic!("Couldn't allocate a new StringBuilder chunk");
            }

            allocated.write(chunk);
            allocated
        }
    }

    /// Collects all data from chunks into one u8 buffer
    /// Very expensive operation
    fn write_to_slice(&self, buffer: &mut [u8]) {
        let chunks = self.get_chunks_count();
        let mut remaining_chunk_size = if chunks * STRING_CHUNK_BYTES_LEN == self.bytes_count { 
            STRING_CHUNK_BYTES_LEN 
        } else { 
            self.bytes_count - (chunks - 1) * STRING_CHUNK_BYTES_LEN 
        };
        let mut current_chunk = self.last_chunk;
        let mut index = self.bytes_count;

        unsafe {
            while !current_chunk.is_null() {
                index -= remaining_chunk_size;
                std::ptr::copy_nonoverlapping(
                    (*current_chunk).bytes.as_ptr(),
                    &mut buffer[index] as *mut u8,
                    remaining_chunk_size
                );

                current_chunk = (*current_chunk).prev.cast_mut();
                remaining_chunk_size = STRING_CHUNK_BYTES_LEN;
            }
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
            last_chunk: std::ptr::null_mut(),
            bytes_count: 0,
        }
    }

    /// Adds a string slice into the StringBuilder buffer
    /// As a result, it concatenates the argument to the previous added strings
    pub fn push_str(&mut self, string: &str) {
        if string.is_empty() { return; }

        let chunks = self.get_chunks_count();
        let mut chunk_left_size = chunks * STRING_CHUNK_BYTES_LEN - self.bytes_count;
        let mut bytes_left = string.len();

        while bytes_left != 0 {
            if chunk_left_size == 0 {
                chunk_left_size = STRING_CHUNK_BYTES_LEN;
                self.last_chunk = Self::allocate_new_chunk(StringChunk {
                    bytes: [0; STRING_CHUNK_BYTES_LEN],
                    prev: self.last_chunk
                });
            }

            let chunk_bytes_to_fill = chunk_left_size.min(bytes_left);

            unsafe {
                std::ptr::copy_nonoverlapping(
                    &string.as_bytes()[string.len() - bytes_left] as *const u8,
                    &mut (*self.last_chunk).bytes[STRING_CHUNK_BYTES_LEN - chunk_left_size] as *mut u8,
                    chunk_bytes_to_fill
                );
            }

            bytes_left -= chunk_bytes_to_fill;
            chunk_left_size -= chunk_bytes_to_fill;
        }

        self.bytes_count += string.len();
    }
}

impl From<&str> for StringBuilder {
    fn from(string: &str) -> Self {
        let string_bytes = string.as_bytes();
        let bytes_count = string_bytes.len();
        let chunks_count = (bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN;

        let mut last_chunk: *mut StringChunk = std::ptr::null_mut();

        // Loop for splitting our string into chunks
        for chunk_index in 0..chunks_count {
            let current_chunk_start = chunk_index * STRING_CHUNK_BYTES_LEN;
            let current_chunk_end = (current_chunk_start + STRING_CHUNK_BYTES_LEN - 1).min(bytes_count - 1);
            let mut current_string_chunk = StringChunk {
                bytes: [0; STRING_CHUNK_BYTES_LEN],
                prev: last_chunk
            };

            for chunk_byte_index in 0..=(current_chunk_end - current_chunk_start) {
                current_string_chunk.bytes[chunk_byte_index] = string_bytes[chunk_byte_index + current_chunk_start];
            }

            last_chunk = Self::allocate_new_chunk(current_string_chunk);
        }

        Self { 
            last_chunk,
            bytes_count,
        }
    }
}

impl Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bytes_count == 0 { return f.write_str(""); }

        let mut buf = vec![0u8; self.bytes_count];
        self.write_to_slice(&mut buf);

        unsafe {
            f.write_str(&String::from_utf8_unchecked(buf))
        }
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
        let mut current = self.last_chunk;
        let layout = Layout::new::<StringChunk>();

        while !current.is_null() {
            unsafe {
                let prev = (*current).prev.cast_mut();
                std::ptr::drop_in_place(current);
                dealloc(current as *mut u8, layout);
                current = prev;
            }
        }
    }
}
