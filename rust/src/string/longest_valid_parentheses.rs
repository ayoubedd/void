fn check_valid_range_len(s: &str) -> i32 {
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    let mut stack: Vec<Pair<char, usize>> = Vec::with_capacity(s.len());
    let mut len: i32 = 0;

    for (i, el) in s.chars().enumerate() {
        if el == '(' {
            stack.push(Pair {
                first: el,
                second: i,
            });
            len += 1;
            continue;
        }

        if stack.len() > 0 {
            stack.pop();
        } else {
            return len;
        }

        len += 1;
    }

    if stack.len() > 0 {
        return len - (s.len() - stack.first().unwrap().second) as i32;
    }

    len
}

pub fn longest_valid_parentheses(s: &str) -> i32 {
    let mut longest: i32 = 0;

    let mut i: usize = 0;

    while i < s.len() {
        let len = check_valid_range_len(&s[i..]);
        if len > longest {
            longest = len;
            i += len as usize;
        }
        i += 1;
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let str = "()(((())))()";
        let longest = longest_valid_parentheses(str);
        assert_eq!(longest, 12);
    }
}
