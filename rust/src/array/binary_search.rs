fn binary_search<T: PartialOrd>(input: &[T], target: T) -> Option<&T> {
    if input.len() == 0 {
        return None;
    }
    if input.len() == 1 {
        if input[0] == target {
            return Some(&input[0]);
        }
        return None;
    }

    let mid = input.len() / 2;

    if input[mid] == target {
        return Some(&input[mid]);
    }

    if input[mid] > target {
        return binary_search(&input[..mid], target);
    }

    return binary_search(&input[mid..], target);
}

fn binary_search_iter<T: PartialOrd>(input: &[T], target: T) -> Option<&T> {
    if input.len() == 0 {
        return None;
    }
    if input.len() == 1 {
        if input[0] == target {
            return Some(&input[0]);
        }
        return None;
    }

    let mut high = input.len() - 1;
    let mut low = 0;

    while low <= high {
        let mid = (low + high) / 2;

        if input[mid] > target {
            high = mid - 1;
        } else if input[mid] < target {
            low = mid + 1;
        } else {
            return Some(&input[mid]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let numbers = [1, 2, 3, 4, 5];
        let result = binary_search(&numbers, 2);
        let result_iter = binary_search_iter(&numbers, 2);
        assert_eq!(result, Some(&numbers[1]));
        assert_eq!(result_iter, Some(&numbers[1]));
    }

    #[test]
    fn missing() {
        let numbers = [1, 2, 3, 4, 5];
        let result = binary_search(&numbers, 20);
        let result_iter = binary_search_iter(&numbers, 20);
        assert_eq!(result, None);
        assert_eq!(result_iter, None);
    }

    #[test]
    fn empty() {
        let numbers = [];
        let result = binary_search(&numbers, 20);
        let result_iter = binary_search_iter(&numbers, 20);
        assert_eq!(result, None);
        assert_eq!(result_iter, None);
    }

    #[test]
    fn one_value_fail() {
        let numbers = [1];
        let result = binary_search(&numbers, 2);
        let result_iter = binary_search_iter(&numbers, 2);
        assert_eq!(result, None);
        assert_eq!(result_iter, None);
    }

    #[test]
    fn one_value() {
        let numbers = [1];
        let result = binary_search(&numbers, 1);
        let result_iter = binary_search_iter(&numbers, 1);
        assert_eq!(result, Some(&numbers[0]));
        assert_eq!(result_iter, Some(&numbers[0]));
    }

    #[test]
    fn characters() {
        let numbers = ['a', 'b', 'c', 'd'];
        let result = binary_search(&numbers, 'c');
        let result_iter = binary_search_iter(&numbers, 'c');
        assert_eq!(result, Some(&numbers[2]));
        assert_eq!(result_iter, Some(&numbers[2]));
    }
}
