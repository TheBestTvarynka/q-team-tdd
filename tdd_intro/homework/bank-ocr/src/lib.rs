use std::io;


pub fn parse_account_number<T: AsRef<str>>(data: T) -> io::Result<Vec<[u8; 9]>> {
    let data = data.as_ref();

    let mut account_numbers = Vec::new();

    let mut lines = data.lines();

    loop {
        let number_top = if let Some(line) = lines.next() {
            line
        } else {
            break;
        };

        let number_middle = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid lines amount"))?;
        let number_bottom = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid lines amount"))?;

        println!("Invalid lines len: {} {} {}", number_top.len(), number_middle.len(), number_bottom.len());

        if number_top.len() != 27 || number_middle.len() != 27 || number_bottom.len() != 27 {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid lines len: {} {} {}", number_top.len(), number_middle.len(), number_bottom.len())));
        }

        let mut account_number = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        for i in 0..9 {
            let index = i * 3;
            let digit_top = &number_top[index..index + 3];
            let digit_middle = &number_middle[index..index + 3];
            let digit_bottom = &number_bottom[index..index + 3];

            println!("`{}`", digit_top);
            println!("`{}`", digit_middle);
            println!("`{}`", digit_bottom);

            match (digit_top, digit_middle, digit_bottom) {
                (" _ ", "| |", "|_|") => account_number[i] = 0,
                ("   ", "  |", "  |") => account_number[i] = 1,
                (" _ ", " _|", "|_ ") => account_number[i] = 2,
                (" _ ", " _|", " _|") => account_number[i] = 3,
                ("   ", "|_|", "  |") => account_number[i] = 4,
                (" _ ", "|_ ", " _|") => account_number[i] = 5,
                (" _ ", "|_ ", "|_|") => account_number[i] = 6,
                (" _ ", "  |", "  |") => account_number[i] = 7,
                (" _ ", "|_|", "|_|") => account_number[i] = 8,
                (" _ ", "|_|", " _|") => account_number[i] = 9,
                _ => return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid number:\n{}\n{}\n{}", digit_top, digit_middle, digit_bottom))),
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
        assert_eq!(vec![[1, 2, 3, 4, 5, 6, 7, 8, 9]], parse_account_number(include_str!("../assets/simple.txt")).unwrap());
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
