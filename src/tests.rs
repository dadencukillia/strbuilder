// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

use crate::StringBuilder;

#[test]
fn test_string_builder_short_string() {
    let string_builder = StringBuilder::from("Hello, world!");
    let string_result = string_builder.to_string();

    assert_eq!(string_result, "Hello, world!");
}

#[test]
fn test_string_builder_long_string() {
    let long_string = "Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!";
    let string_builder = StringBuilder::from(long_string);
    let string_result = string_builder.to_string();

    assert_eq!(string_result, long_string);
}

#[test]
fn test_string_builder_string_128_len() {
    let long_string = "Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, worl";
    let string_builder = StringBuilder::from(long_string);
    let string_result = string_builder.to_string();

    assert_eq!(string_result, long_string);
}

#[test]
fn test_string_push_short_strings_method() {
    let mut string_builder = StringBuilder::new();

    string_builder.push_str("Hello");
    string_builder.push_str(",");
    string_builder.push_str(" ");
    string_builder.push_str("World");
    string_builder.push_str("!");

    let string_result = string_builder.to_string();

    assert_eq!(string_result, "Hello, World!");
}

#[test]
fn test_string_push_lot_strings_method() {
    let mut string_builder = StringBuilder::new();
    let mut string_to_check = String::new();

    for _ in 0..20 {
        string_builder.push_str("Hello, world!");
        string_to_check.push_str("Hello, world!");
    }

    let string_result = string_builder.to_string();

    assert_eq!(string_result, string_to_check);
}

#[test]
fn test_string_push_64_strings_method() {
    let mut string_builder = StringBuilder::new();
    let mut string_to_check = String::new();

    for _ in 0..64 {
        string_builder.push_str("Hello, world!");
        string_to_check.push_str("Hello, world!");
    }

    let string_result = string_builder.to_string();

    assert_eq!(string_result, string_to_check);
}

#[test]
fn test_string_push_empty_strings_method() {
    let mut string_builder = StringBuilder::new();

    for _ in 0..64 {
        string_builder.push_str("");
    }

    let string_result = string_builder.to_string();

    assert_eq!(string_result, "");
}

#[test]
fn test_string_from_with_push() {
    let mut string_builder = StringBuilder::from("Hello,");

    string_builder.push_str(" ");
    string_builder.push_str("world!");

    let string_result = string_builder.to_string();

    assert_eq!(string_result, "Hello, world!");
}

#[test]
fn test_string_long_from_with_long_push() {
    let mut string_builder = StringBuilder::from("Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!");

    string_builder.push_str("Hello, world!Hello, world!Hello, world!Hello, world!Hello, world");

    let string_result = string_builder.to_string();

    assert_eq!(string_result, "Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world!Hello, world");
}
