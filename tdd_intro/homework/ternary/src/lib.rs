pub const BASE: u32 = 3;

pub fn to_decimal<T: AsRef<str>>(data: T) -> u32 {
    let mut number = 0;

    for (index, digit) in data.as_ref().chars().rev().enumerate() {
        let digit = match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => return 0,
        };

        number += digit * BASE.pow(index as u32);
    }

    number
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

    #[test]
    fn invalid_input() {
        assert_eq!(0, to_decimal("132512"));
        assert_eq!(0, to_decimal("1205"));
        assert_eq!(0, to_decimal("6000"));
        assert_eq!(0, to_decimal("3201022"));
    }
}
