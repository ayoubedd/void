use std::{cmp::PartialOrd, vec};

fn merge<T: PartialOrd + Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    let mut ai: usize = 0;
    let mut bi: usize = 0;

    loop {
        if ai < a.len() && bi < b.len() {
            if a[ai] < b[bi] {
                result.push(a[ai].clone());
                ai += 1;
            } else {
                result.push(b[bi].clone());
                bi += 1;
            }
        } else if ai < a.len() && bi >= b.len() {
            result.push(a[ai].clone());
            ai += 1;
        } else {
            result.push(b[bi].clone());
            bi += 1;
        }

        if ai >= a.len() && bi >= b.len() {
            break;
        }
    }

    result
}

fn merge_sort<T: Clone + PartialOrd>(set: &[T]) -> Vec<T> {
    if set.len() == 0 {
        return vec![];
    }

    if set.len() == 1 {
        return vec![set[0].clone()];
    }

    let (a, b) = {
        let half = set.len() / 2;

        let a = merge_sort(&set[..half]);
        let b = merge_sort(&set[half..]);

        (a, b)
    };

    merge(&a, &b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut nums = [2, 1, 4, 3];
        let result = merge_sort(&mut nums);
        assert_eq!(result, [1, 2, 3, 4]);
    }

    #[test]
    fn empty() {
        let mut nums: [i32; 0] = [];
        let result = merge_sort(&mut nums);
        assert_eq!(result, []);
    }
}
