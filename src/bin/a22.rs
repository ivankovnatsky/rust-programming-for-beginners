// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

struct Numbers {
    n: i32,
    lower: i32,
    upper: i32,
}

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        return lower;
    } else if n > upper {
        return upper;
    }

    return n;
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_clamp_1() {
        let numbers = Numbers {
            n: 20,
            lower: 10,
            upper: 21,
        };

        let result = clamp(numbers.n, numbers.lower, numbers.upper);
        let expected = numbers.n;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_all_clamp_2() {
        let numbers = Numbers {
            n: 50,
            lower: 25,
            upper: 50,
        };

        let result = clamp(numbers.n, numbers.lower, numbers.upper);
        let expected = numbers.n;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_div_1() {
        let a = 50;
        let b = 10;

        let result = div(a, b);
        let expected = Some(5);
        assert_eq!(result, expected);
    }

    #[test]
    fn check_div_2() {
        let a = 1;
        let b = 10;

        let result = div(a, b);
        let expected = Some(0);
        assert_eq!(result, expected);
    }

    #[test]
    fn check_div_3() {
        let a = 1;
        let b = 0;

        let result = div(a, b);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_concat_1() {
        let s1 = "String1";
        let s2 = "String2";

        let result = concat(s1, s2);
        let expected = format!("{}{}", s1.to_owned(), s2.to_owned());
        assert_eq!(result, expected);
    }

    #[test]
    fn check_concat_2() {
        let s1 = "Aasdfzxcv";
        let s2 = "Pasodfij";

        let result = concat(s1, s2);
        let expected = format!("{}{}", s1.to_owned(), s2.to_owned());
        assert_eq!(result, expected);
    }
}
