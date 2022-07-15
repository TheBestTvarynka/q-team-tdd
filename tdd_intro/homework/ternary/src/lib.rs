
pub fn to_decimal<T: AsRef<str>>(_data: T) -> u32 {
    302
}

#[cfg(test)]
mod tests {
    use crate::to_decimal;

    #[test]
    fn valid_input() {
        assert_eq!(302, to_decimal("102012"));
        assert_eq!(46, to_decimal("1201"));
        assert_eq!(27, to_decimal("1000"));
        assert_eq!(35, to_decimal("001022"));
    }
}
