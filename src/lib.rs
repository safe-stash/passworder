pub mod password {
    use rand::{thread_rng, Rng};

    const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const DIGIT: &str = "0123456789";
    const PUNCTUATION: &str = "-_!.:";
    const SPECIAL: &str = "\"#$%&'*+,/;=?@\\^`|~";
    const BRACKET: &str = "[]{}()<>";

    pub fn generate() -> String {
        let all = format!("{UPPER_CASE}{LOWER_CASE}{DIGIT}{PUNCTUATION}{SPECIAL}{BRACKET}");
        let mut pass = String::new();
        loop {
            for _n in 1..16 {
                pass.push(
                    all.chars()
                        .nth(thread_rng().gen_range(0..=(all.len() - 1)))
                        .unwrap(),
                );
            }
            if is_strong(&pass) {
                return pass;
            }
            pass = String::from("");
        }
    }

    fn is_strong(pass: &str) -> bool {
        if pass.len() < 8 {
            return false;
        }
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        let mut has_symbol = false;
        for n in 0..pass.len() {
            match pass.chars().nth(n).unwrap() {
                c if c.is_ascii_uppercase() => has_upper = true,
                c if c.is_ascii_lowercase() => has_lower = true,
                c if c.is_ascii_digit() => has_digit = true,
                c if PUNCTUATION.contains(c) || SPECIAL.contains(c) || BRACKET.contains(c) => {
                    has_symbol = true
                }
                _ => {}
            }
        }
        has_upper && has_lower && has_digit && has_symbol
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let pass = generate();
            assert_eq!(is_strong(&pass), true);
        }
    }
}
