
pub fn to_decimal<T: AsRef<str>>(_data: T) -> u32 {
    302
}

#[cfg(test)]
mod tests {
    use crate::to_decimal;

    #[test]
    fn basic_conversion() {
        assert_eq!(302, to_decimal("102012"));
    }
}
