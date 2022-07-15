
pub fn is_leap(year: u32) -> bool {
    year % 4 == 0
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
    fn not_leap_100th() {
        assert!(!is_leap(1700));
        assert!(!is_leap(1800));
        assert!(!is_leap(1900));
    }

    #[test]
    fn leap_400() {
        assert!(is_leap(1600));
        assert!(is_leap(2000));
    }

    #[test]
    fn basic_leap() {
        assert!(is_leap(2020));
        assert!(is_leap(1996));
    }
}
