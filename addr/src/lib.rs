#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // unimplemented!()
        assert_eq!(4, add(2));
    }
}

pub fn add(i: i32) -> i32 {
    i + 2
}
