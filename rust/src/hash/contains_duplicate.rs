use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash_set: HashSet<i32> = HashSet::new();

    for i in nums {
        if hash_set.get(&i).is_some() {
            return true;
        }

        hash_set.insert(i);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    fn basic2() {
        let nums = vec![1, 2, 3, 4, 4];
        assert_eq!(contains_duplicate(nums), true);
    }
}
