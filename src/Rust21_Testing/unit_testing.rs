pub fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

pub fn wrong_add<T: std::ops::Sub<Output=T>>(a: T, b: T) -> T {
    a - b
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_wrong() {
        assert_eq!(wrong_add(1, 2), 3);
    }
}

// unable to execute `cargo test` because project structure

// unit test with `Result` return
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}

