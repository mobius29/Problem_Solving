use std::io::{stdin, stdout, BufWriter, Write};

fn read(line: &mut String) {
    stdin().read_line(line).expect("Failed to read line");
}

fn main() {
    let stdout = stdout();
    let buf_write = &mut BufWriter::new(stdout.lock());

    let mut line = String::new();
    read(&mut line);

    let n: usize = line.trim().parse().unwrap();
    let mut counts: [usize; 10001] = [0; 10001];
    for _i in 0..n {
        let mut num = String::new();
        read(&mut num);

        let num: usize = num.trim().parse().unwrap();

        counts[num] += 1;
    }

    for i in 1..10001 {
        for _j in 0..counts[i] {
            writeln!(buf_write, "{}", i).ok();
        }
    }
}
