mod digit_literals;

use std::io;

use digit_literals::{EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO};

// 3 /*digit width*/ * 9 /*digits amount*/
const LINE_LEN: usize = 3 * 9;

pub fn parse_account_number<T: AsRef<str>>(data: T) -> io::Result<Vec<[u8; 9]>> {
    let data = data.as_ref();

    let mut account_numbers = Vec::new();

    let mut lines = data.lines();

    while let Some(number_top) = lines.next() {
        let number_middle = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid lines amount"))?;
        let number_bottom = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid lines amount"))?;

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

        let mut account_number = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        for (index, item) in account_number.iter_mut().enumerate() {
            let index = index * 3;

            match [
                &number_top[index..index + 3],
                &number_middle[index..index + 3],
                &number_bottom[index..index + 3],
            ] {
                ZERO => *item = 0,
                ONE => *item = 1,
                TWO => *item = 2,
                THREE => *item = 3,
                FOUR => *item = 4,
                FIVE => *item = 5,
                SIX => *item = 6,
                SEVEN => *item = 7,
                EIGHT => *item = 8,
                NINE => *item = 9,
                data => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Invalid number: {:?}", data),
                    ))
                }
            };
        }

        account_numbers.push(account_number);
    }

    Ok(account_numbers)
}

#[cfg(test)]
mod tests {
    use crate::parse_account_number;

    #[test]
    fn simple() {
        assert_eq!(
            vec![[1, 2, 3, 4, 5, 6, 7, 8, 9]],
            parse_account_number(include_str!("../assets/simple.txt")).unwrap()
        );
    }

    #[test]
    fn empty() {
        assert_eq!(Vec::<[u8; 9]>::new(), parse_account_number("").unwrap());
    }

    #[test]
    fn multiple_numbers() {
        assert_eq!(
            vec![
                [1, 2, 3, 4, 5, 6, 7, 8, 9],
                [9, 2, 3, 7, 5, 6, 7, 3, 9],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            parse_account_number(include_str!("../assets/multiple_inputs.txt")).unwrap()
        );
    }
}
