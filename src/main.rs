use std::{
    io::{BufWriter, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let first = 'a' as u32;
    let last = 'z' as u32; // inclusive
    assert!(last > first);

    let newline_prob = 10; // percentage

    let out = std::io::stdout().lock();
    let mut out = BufWriter::new(out);

    loop {
        let result = write!(out, "{}", next_char(first, last, newline_prob));

        if let Err(err) = result {
            return if err.kind() == std::io::ErrorKind::BrokenPipe {
                ExitCode::from(0)
            } else {
                ExitCode::from(1)
            };
        }
    }
}

fn next_char(first: u32, last: u32, newline_prob: u8) -> char {
    if fastrand::u8(0..100) < newline_prob {
        '\n'
    } else {
        char::from_u32(fastrand::u32(first..=last)).unwrap()
    }
}
