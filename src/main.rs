mod parse {
    use std::io;
    use std::str;
    use std::num::ParseFloatError;

    #[derive(Debug)]
    pub enum Error {
        Exhausted,
        Io(io::Error),
        InvalidDomain(u32, u32),
        ParseFloat(ParseFloatError),
    }

    impl From<ParseFloatError> for Error {
        fn from(err: ParseFloatError) -> Self {
            Error::ParseFloat(err)
        }
    }
    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::Io(err)
        }
    }

    const DIGIT_POWER: [u32; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];

    pub fn consume_until(input: &[u8], stop_byte: u8) -> Result<(&[u8], &[u8]), Error> {
        let (input, remainder) = input.split_at(input
            .iter()
            .position(|b| *b == stop_byte)
            .ok_or(Error::Exhausted)?);

        Ok((input, &remainder[1..]))
    }

    pub fn float_stop_at(input: &[u8], stop_byte: u8) -> Result<(f32, &[u8]), Error> {
        let (float, cursor) = consume_until(input, stop_byte)?;
        let float: f32 = unsafe { str::from_utf8_unchecked(float) }.parse()?;
        Ok((float, cursor))
    }

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
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::str;

pub const MAX_LENGTH: u32 = 10_000;
pub const MAX_RECIPES: u32 = 1000;
pub const MAX_INGREDIENTS: u32 = 20;
pub const MAX_PORTIONS: u32 = 12;
pub const MAX_DESIRED_PORTIONS: u32 = 1000;

fn main() -> Result<(), Error> {
    let mut buf = Vec::with_capacity(1024 * 1024);
    stdin().read_to_end(&mut buf)?;
    let mut writer = BufWriter::with_capacity(64 * 1024, stdout());

    let mut ingredients = Vec::with_capacity(MAX_INGREDIENTS as usize);
    let (num_recipes, mut cursor) = parse::digit_stop_at(&buf, MAX_RECIPES, b'\n')?;
    for current_recipe in 0..num_recipes {
        let (num_ingredients, ncursor) = parse::digit_stop_at(cursor, MAX_INGREDIENTS, b' ')?;
        let (portions, ncursor) = parse::digit_stop_at(ncursor, MAX_PORTIONS, b' ')?;
        let (desired_portions, mut ncursor) =
            parse::digit_stop_at(ncursor, MAX_DESIRED_PORTIONS, b'\n')?;
        let scaling_for_100percent_ingredient = desired_portions as f32 / portions as f32;

        writeln!(writer, "Recipe # {}", current_recipe + 1)?;
        ingredients.clear();
        let mut master_weight = 0.0_f32;

        cursor = ncursor;
        for _ in 0..num_ingredients {
            let (name, ncursor) = parse::consume_until(cursor, b' ')?;
            let (weight, ncursor) = parse::float_stop_at(ncursor, b' ')?;
            let (percentage, ncursor) = parse::float_stop_at(ncursor, b'\n')?;
            ingredients.push((name, percentage));
            if percentage == 100.0 {
                master_weight = weight * scaling_for_100percent_ingredient;
            }
            cursor = ncursor;
        }

        for (name, percentage) in &ingredients {
            writeln!(
                writer,
                "{} {:.1}",
                unsafe { str::from_utf8_unchecked(name) },
                master_weight * (percentage / 100.0)
            )?;
        }

        writeln!(writer, "----------------------------------------")?;
    }
    Ok(())
}
