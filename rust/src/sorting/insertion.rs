fn insertion_sort<T: std::cmp::PartialOrd>(numbers: &mut [T]) {
    if numbers.len() == 1 {
        return;
    }

    let mut i: usize = 1;
    while i < numbers.len() {
        if numbers[i] > numbers[i - 1] {
            i += 1;
            continue;
        }

        let mut ii = i;
        loop {
            numbers.swap(ii, ii - 1);

            ii -= 1;

            if ii == 0 || numbers[ii] > numbers[ii - 1] {
                break;
            }
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_numbers() {
        let mut nums = [1, 2, 100, 3, 0, -1, -200];

        insertion_sort(&mut nums);
        assert_eq!(nums, [-200, -1, 0, 1, 2, 3, 100]);
    }

    #[test]
    fn sort_charaters() {
        let mut chars = ['c', 'b', 'a'];

        insertion_sort(&mut chars);
        assert_eq!(chars, ['a', 'b', 'c']);
    }
}
