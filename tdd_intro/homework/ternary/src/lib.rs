use core::num;


pub fn to_decimal<T: AsRef<str>>(data: T) -> u32 {
    let mut number = 0;
    let mut index = 0;

    for digit in data.as_ref().chars().rev() {
        let digit = match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };

        number += digit * 3_u32.pow(index);
        index += 1;
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
}
