fn main() {
    let first = 'a' as u32;
    let last = 'z' as u32; // inclusive
    assert!(last > first);

    let newline_prob = 10; // percentage

    loop {
        if fastrand::u8(0..100) < newline_prob {
            println!();
        } else {
            print!("{}", char::from_u32(fastrand::u32(first..=last)).unwrap());
        }
    }
}
