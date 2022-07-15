use std::collections::HashMap;

pub fn count_words<T: AsRef<str>>(data: T) -> HashMap<String, usize> {
    let data = data.as_ref();
    let mut words = HashMap::new();

    if data.is_empty() {
        return words;
    }

    for word in data.split(' ') {
        match words.get_mut(word) {
            Some(count) => *count = *count + 1,
            None => {
                words.insert(word.into(), 1);
            }
        };
    }

    words
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::count_words;

    #[test]
    fn basic_count() {
        let mut words = HashMap::new();
        words.insert("pavlo".into(), 1);
        words.insert("myroniuk".into(), 1);

        let result_words = count_words("pavlo myroniuk");

        assert_eq!(words, result_words);
    }

    #[test]
    fn count_words_with_different_cases() {
        let mut words = HashMap::new();
        words.insert("pavlo".into(), 1);
        words.insert("myroniuk".into(), 1);
        words.insert("yaroslavovych".into(), 2);

        let result_words = count_words("Pavlo MyronIUk YarosLavovych YaroslavoVych");

        assert_eq!(words, result_words);
    }

    #[test]
    fn empty_input() {
        let words = HashMap::new();

        let result_words = count_words("");
        assert_eq!(words, result_words);
    }

    #[test]
    fn long_input() {
        let mut words = HashMap::new();
        words.insert("acceptation".into(), 1);
        words.insert("of".into(), 3);
        words.insert("word".into(), 1);
        words.insert("world".into(), 1);
        words.insert("kind".into(), 1);
        words.insert("was".into(), 1);
        words.insert("he".into(), 1);
        words.insert("a".into(), 2);
        words.insert("in".into(), 1);
        words.insert("zoologist".into(), 1);
        words.insert("cuvier".into(), 1);
        words.insert("new".into(), 1);
        words.insert("then".into(), 1);
        words.insert("the".into(), 3);
        words.insert("entire".into(), 1);

        let result_words = count_words("was he then a zoologist in the entire acceptation of the word a kind of cuvier of the new world");

        assert_eq!(words, result_words);
    }
}
