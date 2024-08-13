fn simplify_path(path: String) -> String {
    let splited = path.split("/");
    let mut result: Vec<String> = Vec::new();

    for el in splited {
        if el.len() == 0 {
            continue;
        }
        match el {
            ".." => {
                result.pop();
            }
            "." => {}
            _ => {
                result.push(el.to_string());
            }
        }
    }

    "/".to_string() + &result.join("/")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let str = String::from("/.../a/../b/c/../d/./");
        let result = simplify_path(str);
        assert_eq!("/.../b/d", result);
    }
}
