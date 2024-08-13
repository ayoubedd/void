pub fn sorted_squared_array(array: &[i32]) -> Vec<i32> {
    let mut squared: Vec<i32> = vec![0; array.len()];

    if array.len() == 0 {
        return squared;
    }

    let mut start: usize = 0;
    let mut end: usize = array.len() - 1;
    let mut i: usize = end;

    loop {
        if array[start].abs() >= array[end].abs() {
            // start is bigger
            squared[i] = array[start].pow(2);
            start += 1;
        } else {
            // end is bigger
            squared[i] = array[end].pow(2);
            end -= 1;
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    squared
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = [-10, 1, 2, 3, 4, 5];
        let result = sorted_squared_array(&input);
        assert_eq!(&result[..], [1, 4, 9, 16, 25, 100]);
    }

    #[test]
    fn empty() {
        let input = [];
        let result = sorted_squared_array(&input);
        assert_eq!(&result[..], []);
    }
}
