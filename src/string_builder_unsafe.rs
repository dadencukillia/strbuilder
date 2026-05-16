// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

use std::{
    alloc::{Layout, alloc, dealloc},
    fmt::Display
};

const STRING_CHUNK_BYTES_LEN: usize = 64 - 8;

#[repr(align(64))]
struct StringChunk {
    bytes: [u8; STRING_CHUNK_BYTES_LEN],
    prev: *const StringChunk
}

pub struct StringBuilder {
    last_chunk: *mut StringChunk,
    bytes_count: usize,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self {
            last_chunk: std::ptr::null_mut(),
            bytes_count: 0,
        }
    }

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

    pub fn push_str(&mut self, string: &str) {
        if string.is_empty() { return; }

        let mut chunks = (self.bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN;

        if chunks == 0 {
            self.last_chunk = Self::allocate_new_chunk(StringChunk {
                bytes: [0; STRING_CHUNK_BYTES_LEN],
                prev: std::ptr::null_mut()
            });

            chunks = 1;
        }

        let mut chunk_left_size = chunks * STRING_CHUNK_BYTES_LEN - self.bytes_count;
        let mut bytes_iter = string.bytes();

        while let Some(string_byte) = bytes_iter.next() {
            if chunk_left_size == 0 {
                chunk_left_size = STRING_CHUNK_BYTES_LEN;
                self.last_chunk = Self::allocate_new_chunk(StringChunk {
                    bytes: [0; STRING_CHUNK_BYTES_LEN],
                    prev: self.last_chunk
                });
            }

            unsafe {
                (*self.last_chunk).bytes[STRING_CHUNK_BYTES_LEN - chunk_left_size] = string_byte;
            }

            chunk_left_size -= 1;
        }

        self.bytes_count += string.len();
    }

    fn write_to_slice(&self, buffer: &mut [u8]) {
        let chunks = (self.bytes_count + STRING_CHUNK_BYTES_LEN - 1) / STRING_CHUNK_BYTES_LEN;
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

    pub fn to_string(&self) -> String {
        if self.bytes_count <= 4096 {
            let mut static_buf: [u8; _] = [0; 4096];
            self.write_to_slice(&mut static_buf);
            String::from_utf8_lossy(&static_buf[..self.bytes_count]).into()
        } else {
            let mut heap_buf: Vec<u8> = vec![0; self.bytes_count];
            self.write_to_slice(&mut heap_buf);
            String::from_utf8_lossy(&heap_buf).into()
        }
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
        f.write_str(&self.to_string())
    }
}

impl Into<String> for StringBuilder {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<String> for StringBuilder {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        Self::new()
    }
}
