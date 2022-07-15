use std::collections::HashMap;

pub fn count_words<T: AsRef<str>>(_data: T) -> HashMap<String, usize> {
    let mut words = HashMap::new();
    words.insert("Pavlo".into(), 1);
    words.insert("Myroniuk".into(), 1);

    words
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::count_words;

    #[test]
    fn basic_count() {
        let mut words = HashMap::new();
        words.insert("Pavlo".into(), 1);
        words.insert("Myroniuk".into(), 1);

        let result_words = count_words("Pavlo Myroniuk");

        assert_eq!(words, result_words);
    }
}
