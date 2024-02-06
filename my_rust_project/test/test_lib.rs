use trust::lib;
use super::*;
use std::io::{self, Write};
use std::io::Cursor;

#[test]
fn test_read_my_name_with_name() {
    let mut input = Cursor::new(b"John\n");
    let mut output = Vec::new();

    // Redirect stdin and stdout
    io::stdin().with_reader(&mut input, |stdin| {
        io::stdout().with_writer(&mut output, |stdout| {
            io::stdout().with_writer(stdout, || {
                read_my_name();
                Ok(())
            })
        })
    }).unwrap();

    // Assert output
    assert_eq!(output, b"Hello, John! Nice to meet you!\n");
}

// Test when the user doesn't input a name
#[test]
fn test_read_my_name_without_name() {
    let mut input = Cursor::new(b"\n");
    let mut output = Vec::new();

    // Redirect stdin and stdout
    io::stdin().with_reader(&mut input, |stdin| {
        io::stdout().with_writer(&mut output, |stdout| {
            io::stdout().with_writer(stdout, || {
                read_my_name();
                Ok(())
            })
        })
    }).unwrap();

    // Assert output
    assert_eq!(output, b"You didn't enter a name!\n");
}