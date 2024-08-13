fn max_area(heights: Vec<i32>) -> i32 {
    let mut max_area: i32 = -1;

    let mut begin: usize = 0;
    let mut end: usize = heights.len() - 1;

    loop {
        let left = heights[begin];
        let right = heights[end];

        let smallest = {
            if left < right {
                begin += 1;
                (left, end - begin + 1)
            } else {
                end -= 1;
                (right, end - begin + 1)
            }
        };

        let area = smallest.0 * smallest.1 as i32;

        if area > max_area {
            max_area = area;
        }

        if begin == end {
            break;
        };
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = max_area(height);
        assert_eq!(result, 49);
    }
}
