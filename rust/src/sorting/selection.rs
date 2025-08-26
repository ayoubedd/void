fn selection_sort<T: std::cmp::PartialOrd>(elements: &mut [T]) {
    let mut i: usize;
    let mut sorted_till_index: usize = 0;

    while sorted_till_index < elements.len() {
        let mut smallest_index = sorted_till_index;

        i = sorted_till_index + 1;
        while i < elements.len() {
            if elements[i] < elements[smallest_index] {
                smallest_index = i;
            }
            i += 1;
        }

        elements.swap(sorted_till_index, smallest_index);
        sorted_till_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut nums = [1, 2, 100, 3, 0, -1, -200];

        selection_sort(&mut nums);
        assert_eq!(nums, [-200, -1, 0, 1, 2, 3, 100]);
    }
}
