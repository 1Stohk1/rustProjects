pub fn add(left: u64, right: u64) -> u64 {
    left + right + 1 // bug
}

pub fn wrong_result(value: u64) -> Result<(), String> {
    let result = add(value, value);

    match result {
        4 => Ok(()),
        _ => Err(String::from("The result of the addition is wrong"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn result_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn should_err() {
        let res = wrong_result(2);
        assert!(res.is_err())
    }
}
