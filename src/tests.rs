// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

use crate::StringBuilder;

#[test]
fn test_from_short_string() {
    let string_builder = StringBuilder::from("Hello, world!");
    let result = string_builder.to_string();

    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_from_long_string() {
    let long_string = "Hello, world!".repeat(10_000);
    let string_builder = StringBuilder::from(long_string.as_str());
    let result = string_builder.to_string();

    assert_eq!(result, long_string);
}

#[test]
fn test_push_short_strings() {
    let mut string_builder = StringBuilder::new();

    string_builder.push_str("Hello");
    string_builder.push_str(",");
    string_builder.push_str(" ");
    string_builder.push_str("World");
    string_builder.push_str("!");

    let result = string_builder.to_string();

    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_push_lot_strings() {
    let mut string_builder = StringBuilder::new();
    let mut expected = String::new();

    for _ in 0..2000 {
        string_builder.push_str("Hello, world!");
        expected.push_str("Hello, world!");
    }

    let result = string_builder.to_string();

    assert_eq!(result, expected);
}

#[test]
fn test_push_empty_strings() {
    let mut string_builder = StringBuilder::new();

    for _ in 0..64 {
        string_builder.push_str("");
    }

    let result = string_builder.to_string();

    assert_eq!(result, "");
}

#[test]
fn test_from_and_push() {
    let mut string_builder = StringBuilder::from("Hello,");

    string_builder.push_str(" ");
    string_builder.push_str("world!");

    let result = string_builder.to_string();

    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_long_from_and_long_push() {
    let mut string_builder = StringBuilder::from("Hello, world!".repeat(500));

    string_builder.push_str(&"Hello, world!".repeat(100));

    let result = string_builder.to_string();

    assert_eq!(result, "Hello, world!".repeat(600));
}

#[test]
fn test_everything_empty() {
    let mut string_builder = StringBuilder::from("");
    string_builder.push_str("");

    assert_eq!(string_builder.to_string(), "");
}

/*****************
 * Tests from AI *
 *****************/

#[test]
fn test_string_builder_unicode_and_emojis() {
    let mut string_builder = StringBuilder::from("Привіт, 👋 ");
    string_builder.push_str("Світ! 🌍");
    
    let string_result = string_builder.to_string();
    assert_eq!(string_result, "Привіт, 👋 Світ! 🌍");
}

#[test]
fn test_string_builder_with_special_characters() {
    let mut string_builder = StringBuilder::new();
    string_builder.push_str("Line 1\n");
    string_builder.push_str("\tTabbed text\r\n");
    string_builder.push_str("Quotes: \"Hello\" and slashes: \\");

    let expected = "Line 1\n\tTabbed text\r\nQuotes: \"Hello\" and slashes: \\";
    assert_eq!(string_builder.to_string(), expected);
}

#[test]
fn test_string_builder_alternating_empty_and_non_empty() {
    let mut string_builder = StringBuilder::new();
    string_builder.push_str("Alpha");
    string_builder.push_str("");
    string_builder.push_str("Beta");
    string_builder.push_str("");
    string_builder.push_str("Gamma");

    assert_eq!(string_builder.to_string(), "AlphaBetaGamma");
}

#[test]
fn test_string_builder_massive_growth() {
    let mut string_builder = StringBuilder::new();
    let mut string_to_check = String::new();
    
    let chunk = "Data chunk 12345! ";
    for _ in 0..2000 {
        string_builder.push_str(chunk);
        string_to_check.push_str(chunk);
    }

    assert_eq!(string_builder.to_string(), string_to_check);
}

#[test]
fn test_string_builder_single_character_pushes() {
    let mut string_builder = StringBuilder::new();
    let chars = vec!["R", "u", "s", "t", " ", "2", "0", "2", "6"];
    
    for c in chars {
        string_builder.push_str(c);
    }

    assert_eq!(string_builder.to_string(), "Rust 2026");
}

#[test]
fn test_string_builder_very_long_initial_from() {
    let base_pattern = "A".repeat(5000);
    let string_builder = StringBuilder::from(base_pattern.as_str());
    
    assert_eq!(string_builder.to_string(), base_pattern);
}

#[test]
fn test_string_builder_push_exact_power_of_two_lengths() {
    let mut string_builder = StringBuilder::new();
    
    let chunk_64 = "a".repeat(64);
    string_builder.push_str(&chunk_64);
    assert_eq!(string_builder.to_string().len(), 64);
    
    string_builder.push_str(&chunk_64);
    assert_eq!(string_builder.to_string().len(), 128);
    
    let chunk_128 = "b".repeat(128);
    string_builder.push_str(&chunk_128);
    assert_eq!(string_builder.to_string().len(), 256);
}

#[test]
fn test_string_builder_multiple_to_string_calls() {
    let mut string_builder = StringBuilder::from("Keep");
    string_builder.push_str(" It");
    
    let res1 = string_builder.to_string();
    let res2 = string_builder.to_string();
    
    assert_eq!(res1, "Keep It");
    assert_eq!(res2, "Keep It");
    assert_eq!(res1, res2);
}

#[test]
fn test_exact_chunk_bounds_from() {
    let exact_chunk = "A".repeat(56);
    let builder = StringBuilder::from(exact_chunk.as_str());
    
    assert_eq!(builder.to_string(), exact_chunk);
}

#[test]
fn test_exact_chunk_bounds_plus_one_from() {
    let exact_chunk_plus_one = "A".repeat(57);
    let builder = StringBuilder::from(exact_chunk_plus_one.as_str());
    
    assert_eq!(builder.to_string(), exact_chunk_plus_one);
}

#[test]
fn test_push_str_exact_chunk_sizes() {
    let mut builder = StringBuilder::new();
    
    builder.push_str(&"A".repeat(56));
    builder.push_str("B");
    
    let expected = format!("{}B", "A".repeat(56));
    assert_eq!(builder.to_string(), expected);
}

#[test]
fn test_push_large_string_crossing_multiple_chunks() {
    let mut builder = StringBuilder::new();
    
    let long_str = "Z".repeat(122);
    builder.push_str(&long_str);
    
    assert_eq!(builder.to_string(), long_str);
}

#[test]
fn test_from_empty_string_then_push() {
    let mut builder = StringBuilder::from("");
    
    builder.push_str("Rust");
    assert_eq!(builder.to_string(), "Rust");
}

#[test]
fn test_display_trait_implementation() {
    let mut builder = StringBuilder::new();
    builder.push_str("Hello Display");
    
    let formatted = format!("{}", builder);
    assert_eq!(formatted, "Hello Display");
}

#[test]
fn test_into_string_conversion() {
    let mut builder = StringBuilder::new();
    builder.push_str("Move semantics");
    
    let result: String = builder.into();
    assert_eq!(result, "Move semantics");
}

#[test]
fn test_complex_asymmetric_pushes() {
    let mut builder = StringBuilder::new();
    
    builder.push_str("A"); 
    builder.push_str(&"B".repeat(55)); 
    let big_push = "C".repeat(56 * 3 + 5);
    builder.push_str(&big_push);
    
    let result = builder.to_string();
    
    let expected = format!("A{}{}", "B".repeat(55), "C".repeat(56 * 3 + 5));
    assert_eq!(result.len(), expected.len());
    assert_eq!(result, expected);
}
