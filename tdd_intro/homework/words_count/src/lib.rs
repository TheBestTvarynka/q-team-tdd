
pub const NAME: &str = "Pavlo Myroniuk";

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn basic_count() {
        let mut words = HashMap::new();
        words.push("Pavlo", 1);
        words.push("Myroniuk", 1);

        let result_words = count_words("Pavlo Myroniuk");

        assert_eq!(words, result_words);
    }
}
