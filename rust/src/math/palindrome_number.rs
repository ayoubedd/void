fn is_palindrome(x: i32) -> bool {
    let mut rx: i32 = 0;

    let mut y = x.abs();
    loop {
        rx = (rx * 10) + y % 10;
        y = y / 10;
        if y == 0 {
            break;
        }
    }

    rx == x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let number = 1234321;
        assert_eq!(true, is_palindrome(number));
    }
}
