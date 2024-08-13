fn bubble_sort<T: std::cmp::PartialOrd>(numbers: &mut [T]) {
    if numbers.len() == 1 {
        return;
    }

    let mut i: usize = numbers.len() - 1;
    let mut j;

    loop {
        j = 0;
        while j < i {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
            j += 1;
        }

        i -= 1;
        if i == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_numbers() {
        let mut nums = [1, 2, 100, 3, 0, -1, -200];

        bubble_sort(&mut nums);
        assert_eq!(nums, [-200, -1, 0, 1, 2, 3, 100]);
    }

    #[test]
    fn sort_charaters() {
        let mut chars = ['c', 'b', 'a'];

        bubble_sort(&mut chars);
        assert_eq!(chars, ['a', 'b', 'c']);
    }
}
