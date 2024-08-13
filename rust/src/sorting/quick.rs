fn quick_sort<T: PartialOrd + Clone>(input: &mut [T]) {
    if input.len() <= 1 {
        return;
    }

    let pivot = input[input.len() - 1].clone();

    let mut i: usize = 0;
    let mut j: usize = 0;

    loop {
        if input[j] <= pivot {
            input.swap(j, i);
            i += 1;
        }

        j += 1;
        if j == input.len() - 1 {
            break;
        }
    }

    input.swap(j, i);

    quick_sort(&mut input[..i]);
    quick_sort(&mut input[i + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut nums = [-100, 2, 1, -10];
        quick_sort(&mut nums);
        assert_eq!(nums, [-100, -10, 1, 2]);
    }
}
