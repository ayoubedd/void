enum CharType {
    Opening,
    Closing,
    Unknow,
}

fn get_char_type(c: &char) -> CharType {
    match c {
        '(' => CharType::Opening,
        '{' => CharType::Opening,
        '[' => CharType::Opening,
        ')' => CharType::Closing,
        '}' => CharType::Closing,
        ']' => CharType::Closing,
        _ => CharType::Unknow,
    }
}

fn is_matching_closer(op: &char, cl: &char) -> bool {
    match op {
        '{' => *cl == '}',
        '(' => *cl == ')',
        '[' => *cl == ']',
        _ => false,
    }
}

fn is_valid(s: &String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for i in s.chars() {
        match get_char_type(&i) {
            CharType::Opening => {
                stack.push(i);
            }
            CharType::Closing => {
                let top = stack.last();
                if let None = top {
                    return false;
                }

                if is_matching_closer(top.unwrap(), &i) == false {
                    return false;
                } else {
                    stack.pop();
                }
            }
            CharType::Unknow => {
                return false;
            }
        }
    }

    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let s = String::from("(){}[]");
        assert_eq!(true, is_valid(&s));
    }

    #[test]
    fn more_complicated() {
        let s = String::from("({{{{{}}}}}){}[]");
        assert_eq!(true, is_valid(&s));
    }

    #[test]
    fn wrong() {
        let s = String::from(")({{{{{}}}}}){}[]");
        assert_eq!(false, is_valid(&s));
    }
}
