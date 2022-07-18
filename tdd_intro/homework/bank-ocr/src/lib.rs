use std::io;


pub fn parse_account_number<T: AsRef<str>>(_data: T) -> io::Result<Vec<[u8; 9]>> {
    Ok(vec![[1, 2, 3, 4, 5, 6, 7, 8, 9]])
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
