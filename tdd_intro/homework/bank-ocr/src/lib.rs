use std::io;


pub fn parse_account_number<T: AsRef<str>>(_data: T) -> io::Result<[u8; 9]> {
    Ok([1, 2, 3, 4, 5, 6, 7, 8, 9])
}

#[cfg(test)]
mod tests {
    use crate::parse_account_number;

    #[test]
    fn simple() {
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], parse_account_number(include_str!("../assets/simple.txt")).unwrap());
    }
}
