
#[cfg(test)]
mod tests {
    #[test]
    fn basic_not_leap() {
        assert!(!is_leap(2021));
    }
}
