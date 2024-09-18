use std::collections::HashMap;

fn fibo_impl(n: usize, map: &mut HashMap<usize, usize>) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    if let Some(value) = map.get(&n) {
        return *value;
    }

    let result = fibo_impl(n - 1, map) + fibo_impl(n - 2, map);
    map.insert(n, result);

    result
}

fn fibo(n: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    return fibo_impl(n, &mut map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let result = fibo(75);
        assert_eq!(result, 1304969544928657 + 806515533049393);
    }
}
