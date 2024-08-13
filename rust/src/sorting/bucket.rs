fn bucket_sort(numbers: &mut [i32]) -> Vec<i32> {
    if numbers.len() <= 1 {
        if numbers.len() == 1 {
            return vec![numbers[0]];
        }
        return Vec::new();
    }

    let offset = numbers.iter().min().unwrap().abs(); // handling negatives
                                                      // since we use values as indexes
                                                      // we need to offset all values by lowest
                                                      // value
    let largest = *numbers.iter().max().unwrap() + offset;

    let mut tmp: Vec<i32> = vec![0; largest as usize + 1]; // probably could done some optimization
                                                           // here but this for education purposes
                                                           // its fine
    let mut result: Vec<i32> = Vec::with_capacity(largest as usize + 1);

    for i in numbers {
        tmp[(*i + offset) as usize] += 1;
    }

    for (idx, value) in tmp.iter().enumerate() {
        for _ in 0..*value {
            result.push(idx as i32 - offset);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut nums = [1, 3, 3, 3, 2, -1, 0];

        let result = bucket_sort(&mut nums);
        assert_eq!(result, [-1, 0, 1, 2, 3, 3, 3].to_vec());
    }

    #[test]
    fn empty() {
        let mut nums = [];

        let result = bucket_sort(&mut nums);
        assert_eq!(result, [].to_vec());
    }
}
