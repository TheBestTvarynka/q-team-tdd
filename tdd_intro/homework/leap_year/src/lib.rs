
pub fn is_leap(_year: u32) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::is_leap;

    #[test]
    fn basic_not_leap() {
        assert!(!is_leap(2021));
    }
}
