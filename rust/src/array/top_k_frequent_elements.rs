use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq: Vec<Vec<i32>> = vec![Vec::new(); nums.len()];
    let mut map: HashMap<i32, u32> = HashMap::new();
    let mut result: Vec<i32> = Vec::with_capacity(k as usize);

    for n in nums {
        if let Some(count) = map.get_mut(&n) {
            *count += 1;
        } else {
            map.insert(n, 1);
        }
    }

    for n in &map {
        freq[*n.1 as usize - 1].push(*n.0);
    }

    drop(map);

    for n in freq.iter_mut().rev() {
        for i in n {
            result.push(*i);
            if result.len() == k as usize {
                break;
            }
        }
        if result.len() == k as usize {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let result = top_k_frequent([1, 1, 1, 1, 2, 3, 2, 4].to_vec(), 3);
        assert_eq!(result, [1, 2, 3].to_vec());
    }
}
