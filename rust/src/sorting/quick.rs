fn partition<T: PartialOrd>(array: &mut [T]) -> usize {
    let mut lo: usize = 0;
    let mut hi: usize = array.len() - 2;
    let pivot_idx: usize = array.len() - 1;

    if array.len() <= 1 {
        return 0;
    }

    loop {
        while lo <= hi && array[lo] <= array[pivot_idx] {
            lo += 1;
        }

        while hi >= lo && array[hi] > array[pivot_idx] {
            if hi == 0 {
                break;
            }
            hi -= 1;
        }

        if lo >= hi || hi <= lo {
            break;
        }

        array.swap(lo, hi);

        lo += 1;
        hi -= 1;
    }

    array.swap(lo, pivot_idx);

    lo
}

fn quick_sort<T: PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let pivot_new_index = partition(array);

    quick_sort(&mut array[..pivot_new_index]);
    quick_sort(&mut array[pivot_new_index + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut nums = [200, -100, 2, 1, -10];
        quick_sort(&mut nums);
        assert_eq!(nums, [-100, -10, 1, 2, 200]);
    }
}
