use std::collections::HashMap;

fn unique_paths(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if r >= rows || c >= cols {
        return 0;
    }

    if r == rows - 1 && c == cols - 1 {
        return 1;
    }

    if let Some(value) = map.get(&(r, c)) {
        return *value;
    }

    let count = unique_paths(r + 1, c, rows, cols, map) + unique_paths(r, c + 1, rows, cols, map);

    map.insert((r, c), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        let result = unique_paths(0, 0, 4, 4, &mut map);
        assert_eq!(result, 20);
    }
}
