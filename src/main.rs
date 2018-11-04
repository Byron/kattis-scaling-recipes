mod parse {
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Io(io::Error),
        InvalidDomain(u32, u32),
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::Io(err)
        }
    }

    const DIGIT_POWER: [u32; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];

    #[inline(always)]
    pub fn digit_stop_at(
        input: &[u8],
        max_value: u32,
        stop_byte: u8,
    ) -> Result<(u32, &[u8]), Error> {
        let mut digits = [0; 7];
        let mut num_digits = 0;
        for d in input
            .iter()
            .take_while(|&&b| b != stop_byte)
            .map(|b| b - b'0')
        {
            digits[num_digits] = d;
            num_digits += 1;
        }

        let res = digits[..num_digits]
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, &b10)| {
                acc + DIGIT_POWER[index] * b10 as u32
            });

        if res > max_value {
            Err(Error::InvalidDomain(res, max_value))
        } else {
            Ok((res, &input[num_digits + 1..]))
        }
    }
}

use parse::Error;
use std::io::{stdin, stdout, Read, Write};
pub const MAX_LENGTH: u32 = 10_000;
pub const MAX_PIECES: u32 = 5_000_000;

fn main() -> Result<(), Error> {
    let mut buf = Vec::with_capacity(1024 * 1024);
    stdin().read_to_end(&mut buf)?;
    Ok(())
}
