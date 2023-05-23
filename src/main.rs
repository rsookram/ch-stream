use std::{
    io::{BufWriter, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let stream = match Stream::from_args(std::env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::from(2);
        }
    };

    let out = std::io::stdout().lock();
    let mut out = BufWriter::new(out);

    for ch in stream {
        let result = write!(out, "{ch}");

        if let Err(err) = result {
            return if err.kind() == std::io::ErrorKind::BrokenPipe {
                ExitCode::from(0)
            } else {
                eprintln!("{}", err);
                ExitCode::from(1)
            };
        }
    }

    ExitCode::from(0)
}

struct Stream {
    first: char,
    last_inclusive: char,
    newline_prob: u8,
}

impl Stream {
    fn from_args(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        args.next();

        let first_arg = args.next().ok_or("first character of range not provided")?;
        let first: char = first_arg
            .parse()
            .map_err(|_| format!("first argument was `{first_arg}`, which is not a char"))?;

        let second_arg = args
            .next()
            .ok_or("second character of range not provided")?;
        let last_inclusive: char = second_arg
            .parse()
            .map_err(|_| format!("second argument was `{second_arg}`, which is not a char"))?;

        if last_inclusive <= first {
            return Err(format!(
                "end of range must be >= start, given {first}..={last_inclusive}"
            ));
        }

        let third_arg = args.next().ok_or("newline probability not provided")?;
        let newline_prob: u8 = third_arg
            .parse()
            .map_err(|_| format!("third argument was `{third_arg}`, which is not a number"))?;
        if !(0..=100).contains(&newline_prob) {
            return Err(format!(
                "newline probability must be given as a percentage (0..=100), given {third_arg}"
            ));
        }

        Ok(Self {
            first,
            last_inclusive,
            newline_prob,
        })
    }
}

impl Iterator for Stream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        Some(if fastrand::u8(0..100) < self.newline_prob {
            '\n'
        } else {
            char::from_u32(fastrand::u32(
                self.first as u32..=self.last_inclusive as u32,
            ))
            .unwrap()
        })
    }
}
