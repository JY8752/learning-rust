use std::ops::Add;

pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn divide(a: i32, b: i32) -> f32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a as f32 / b as f32
}

fn divide2(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a as f32 / b as f32)
}

fn divide3(a: i32, b: i32) -> Result<f32, anyhow::Error> {
    if b == 0 {
        return Err(anyhow::anyhow!("Cannot divide by zero"));
    }
    Ok(a as f32 / b as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add(1, 2) == 3);
        assert_eq!(add(1, 2), 3);
        assert_ne!(add(1, 2), 4);
    }

    #[test]
    #[should_panic]
    fn test_divide() {
        divide(1, 0);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_2() {
        divide(1, 0);
    }

    #[test]
    fn test_divide2() -> Result<(), String> {
        assert_eq!(divide2(4, 2)?, 2.0);
        Ok(())
    }

    #[test]
    fn test_divide3() -> anyhow::Result<()> {
        assert_eq!(divide3(4, 2)?, 2.0);
        Ok(())
    }
}
