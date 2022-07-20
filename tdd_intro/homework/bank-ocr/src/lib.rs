#[rustfmt::skip]
mod digit_literals;

use std::io;

use digit_literals::{EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO};

// 3 /*digit width*/ * 9 /*digits amount*/
const LINE_LEN: usize = 3 * 9;

pub type AccountNumber = [u8; 9];

fn parse_account_number(
    number_top: &str,
    number_middle: &str,
    number_bottom: &str,
) -> io::Result<AccountNumber> {
    let mut account_number = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for (index, digit) in account_number.iter_mut().enumerate() {
        let index = index * 3;

        match [
            &number_top[index..index + 3],
            &number_middle[index..index + 3],
            &number_bottom[index..index + 3],
        ] {
            ZERO => *digit = 0,
            ONE => *digit = 1,
            TWO => *digit = 2,
            THREE => *digit = 3,
            FOUR => *digit = 4,
            FIVE => *digit = 5,
            SIX => *digit = 6,
            SEVEN => *digit = 7,
            EIGHT => *digit = 8,
            NINE => *digit = 9,
            data => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid number: {:?}", data),
                ))
            }
        };
    }

    Ok(account_number)
}

pub fn parse_account_numbers<T: AsRef<str>>(data: T) -> io::Result<Vec<AccountNumber>> {
    let data = data.as_ref();

    let mut account_numbers = Vec::new();

    let mut lines = data.lines();

    while let Some(number_top) = lines.next() {
        let number_middle = lines.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Invalid lines amount: no second line")
        })?;
        let number_bottom = lines.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Invalid lines amount: no third line")
        })?;

        if number_top.len() != LINE_LEN
            || number_middle.len() != LINE_LEN
            || number_bottom.len() != LINE_LEN
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Invalid lines len: {} {} {}",
                    number_top.len(),
                    number_middle.len(),
                    number_bottom.len()
                ),
            ));
        }

        account_numbers.push(parse_account_number(
            number_top,
            number_middle,
            number_bottom,
        )?);
    }

    Ok(account_numbers)
}

#[cfg(test)]
mod tests {
    use crate::parse_account_numbers;

    #[test]
    fn simple() {
        assert_eq!(
            vec![[1, 2, 3, 4, 5, 6, 7, 8, 9]],
            parse_account_numbers(include_str!("../assets/simple.txt")).unwrap()
        );
    }

    #[test]
    fn empty() {
        assert_eq!(Vec::<[u8; 9]>::new(), parse_account_numbers("").unwrap());
    }

    #[test]
    fn multiple_numbers() {
        assert_eq!(
            vec![
                [1, 2, 3, 4, 5, 6, 7, 8, 9],
                [9, 2, 3, 7, 5, 6, 7, 3, 9],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            parse_account_numbers(include_str!("../assets/multiple_inputs.txt")).unwrap()
        );
    }

    #[test]
    fn long_input() {
        assert_eq!(
            486,
            parse_account_numbers(include_str!("../assets/long_input.txt"))
                .unwrap()
                .len()
        );
    }

    #[test]
    fn invalid_lines_amount() {
        assert!(parse_account_numbers(include_str!("../assets/one_line.txt")).is_err());
        assert!(parse_account_numbers(include_str!("../assets/two_lines.txt")).is_err());
    }

    #[test]
    fn invalid_number() {
        assert!(parse_account_numbers(include_str!("../assets/ten_digits.txt")).is_err());
        assert!(parse_account_numbers(include_str!("../assets/eight_digits.txt")).is_err());
        assert!(parse_account_numbers(include_str!("../assets/invalid_digit.txt")).is_err());
    }
}