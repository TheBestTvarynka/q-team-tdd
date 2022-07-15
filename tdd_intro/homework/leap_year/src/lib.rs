
pub fn is_leap(_year: u32) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::is_leap;

    #[test]
    fn basic_not_leap() {
        assert!(!is_leap(2021));
        assert!(!is_leap(1997));
        assert!(!is_leap(1998));
        assert!(!is_leap(1233));
    }

    #[test]
    fn basic_leap() {
        assert!(is_leap(2020));
        assert!(is_leap(1996));
    }
}
