/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut even: bool = false;
    let mut sum: u32 = 0;

    /* strings greater than one char! without taking spaces into account */
    if code.chars().filter(|c| { *c != ' ' }).count() <= 1 {
        return false;
    }

    /* iteration over the reverse string */
    for ch in code.chars().rev() {
        match ch {
            ' ' => continue,
            c if c.is_digit(10) => {
                let mut tmp = c.to_digit(10).unwrap();
                /* Luhn */
                if even == true {
                    tmp *= 2;
                    if tmp > 9 {
                        tmp -= 9;
                    }
                }
                sum += tmp;
                even = !even;
            },
            _ => return false,
        }
    }

    return sum % 10 == 0;
}
