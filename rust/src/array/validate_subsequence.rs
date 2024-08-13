pub fn validate(seq: &[i32], sub: &[i32]) -> bool {
    if sub.len() > seq.len() || sub.len() == 0 {
        return false;
    }

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < seq.len() && j < sub.len() {
        if seq[i] == sub[j] {
            j += 1;
        }
        i += 1;
    }

    j == sub.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same() {
        let seq = [1, 2, 3, 4];
        let sub = &seq[..];
        assert_eq!(validate(&seq, sub), true);
    }

    #[test]
    fn invalid() {
        let seq = [1, 2, 3, 4];
        let sub = [12, 1, 2, 3, 4];
        assert_ne!(validate(&seq, &sub), true);
    }

    #[test]
    fn valid() {
        let seq = [1, 2, 3, 4, 5, 6, 7];
        let sub = [2, 4, 6];
        assert_eq!(validate(&seq, &sub), true);
    }

    #[test]
    fn empty() {
        let seq = [1, 2, 3, 4, 5, 6, 7];
        let sub = [];
        assert_eq!(validate(&seq, &sub), false);
    }

    #[test]
    fn empty_two() {
        let seq = [];
        let sub = [];
        assert_eq!(validate(&seq, &sub), false);
    }

    #[test]
    fn sub_longet_than_seq() {
        let seq = [1];
        let sub = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(validate(&seq, &sub), false);
    }
}
