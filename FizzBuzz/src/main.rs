use std::ops::Add;

fn main() {
    for i in 1..101 {
        println!("{}", get_fizz_buzz(i));
    }
}

fn get_fizz_buzz(i: u8) -> String {
    let mut return_string = String::new();
    let three = i % 3 == 0 || i.to_string().contains('3');
    let five = i % 5 == 0 || i.to_string().contains('5');

    if !three && !five {
        return_string = return_string.add(i.to_string().as_str());
    }
    if three {
        return_string = return_string.add("Fizz");
    }
    if five {
        return_string = return_string.add("Buzz");
    }
    return return_string;
}

#[cfg(test)]
mod tests {
    use crate::get_fizz_buzz;

    #[test]
    fn test_fizz_if_mod_3_equal_0() {
        assert_eq!(get_fizz_buzz(3), "Fizz");
    }

    #[test]
    fn test_fizz_if_number_contains_3() {
        assert_eq!(get_fizz_buzz(13), "Fizz");
    }

    #[test]
    fn test_buzz_if_mod_5_equal_0() {
        assert_eq!(get_fizz_buzz(5), "Buzz");
    }

    #[test]
    fn test_fizz_if_number_contains_5() {
        assert_eq!(get_fizz_buzz(52), "Buzz");
    }

    #[test]
    fn test_fizz_buzz_if_mod_3_equal_0_and_if_mod_5_equal_0() {
        assert_eq!(get_fizz_buzz(15), "FizzBuzz");
    }

    #[test]
    fn test_fizz_buzz_if_number_contains_3_and_if_number_contains_5() {
        assert_eq!(get_fizz_buzz(53), "FizzBuzz");
    }

    #[test]
    fn test_number() {
        assert_eq!(get_fizz_buzz(62), "62");
    }
}
