// https://bamgoesn.github.io/rust-ps-md/misc/fastio.html
use std::io::Write;
use std::{fmt, io};

fn read(line: &mut String) {
    io::stdin().read_line(line).expect("Failed to read line");
}

fn main() {
    // read
    let stdout = io::stdout();
    let buf_write = &mut io::BufWriter::new(stdout.lock());

    // read all line and tokenize by ascii white space
    let mut line = String::new();
    read(&mut line);

    let all: [bool; 21] = [true; 21];
    let empty: [bool; 21] = [false; 21];
    let mut s: [bool; 21] = empty;

    let m: usize = line.trim().parse().unwrap();
    for _i in 0..m {
        let mut line = String::new();
        read(&mut line);

        let str: Vec<&str> = line.as_str().split(" ").collect();
        let instruction = str[0].trim();
        if instruction == "all" {
            s = all;
            continue;
        }

        if instruction == "empty" {
            s = empty;
            continue;
        }

        let x: usize = str[1].trim().parse().unwrap();
        if instruction == "add" {
            s[x] = true;
        }

        if instruction == "remove" {
            s[x] = false;
        }

        if instruction == "check" {
            writeln!(buf_write, "{}", if s[x] { 1 } else { 0 }).ok();
        }

        if instruction == "toggle" {
            s[x] = !s[x];
        }
    }

    // flush buffer
    buf_write.flush().unwrap();
}

