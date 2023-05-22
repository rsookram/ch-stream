use std::{
    env::args,
    io::{BufWriter, Write},
    process::ExitCode,
};

struct Args {
    first: char,
    last_inclusive: char,
    newline_prob: u8,
}

fn main() -> ExitCode {
    let mut args = args();
    args.next();

    let Args {
        first,
        last_inclusive,
        newline_prob,
    } = match parse_args(args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::from(2);
        }
    };

    let out = std::io::stdout().lock();
    let mut out = BufWriter::new(out);

    loop {
        let result = write!(out, "{}", next_char(first, last_inclusive, newline_prob));

        if let Err(err) = result {
            return if err.kind() == std::io::ErrorKind::BrokenPipe {
                ExitCode::from(0)
            } else {
                eprintln!("{}", err);
                ExitCode::from(1)
            };
        }
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Args, String> {
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

    Ok(Args {
        first,
        last_inclusive,
        newline_prob,
    })
}

fn next_char(first: char, last: char, newline_prob: u8) -> char {
    if fastrand::u8(0..100) < newline_prob {
        '\n'
    } else {
        char::from_u32(fastrand::u32(first as u32..=last as u32)).unwrap()
    }
}
