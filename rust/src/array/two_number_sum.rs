use std::collections::HashSet;

pub fn two_sum_hashset(array: &[i32], target: i32) -> bool {
    if array.len() < 1 {
        return false;
    }
    let mut set: HashSet<i32> = HashSet::new();

    for i in array {
        let result = set.get(&(target - i));
        if let Some(_) = result {
            return true;
        } else {
            set.insert(*i);
        }
    }

    false
}

pub fn two_sum_two_pointer(array: &[i32], target: i32) -> bool {
    if array.len() < 1 {
        return false;
    }
    let mut i: usize = 0;
    let mut j: usize = array.len() - 1;
    let mut sorted = Vec::from(array);
    sorted.sort();

    loop {
        let total = sorted[i] + sorted[j];

        if total == target {
            return true;
        } else if total < target {
            i += 1;
        } else {
            j -= 1;
        }

        if i == j {
            break;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let target = 17;
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(two_sum_hashset(&array, target), true);
        assert_eq!(two_sum_two_pointer(&array, target), true);
    }

    #[test]
    fn non_existing() {
        let target = 170;
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(two_sum_hashset(&array, target), false);
        assert_eq!(two_sum_two_pointer(&array, target), false);
    }

    #[test]
    fn empty_array() {
        let target = 170;
        let array = [];
        assert_eq!(two_sum_hashset(&array, target), false);
        assert_eq!(two_sum_two_pointer(&array, target), false);
    }
}
