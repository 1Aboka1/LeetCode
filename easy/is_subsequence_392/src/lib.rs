fn is_subsequence(s: String, t: String) -> bool {
    let mut current_index: usize = 0;
    for charr in s.chars() {
        current_index = match t[current_index..].chars().position(|c| c == charr) {
            Some(x) => {
                let temp = current_index + x;
                if temp + 1 >= current_index {
                    temp + 1
                } else {
                    return false
                }
            },
            None => {
                return false
            },
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = String::from("abc");
        let b = String::from("ahbgdc");
        let correct = true;
        let result = is_subsequence(a, b);

        assert_eq!(correct, result);
    }

    #[test]
    fn example2() {
        let a = String::from("axc");
        let b = String::from("ahbgdc");
        let correct = false;
        let result = is_subsequence(a, b);

        assert_eq!(correct, result);
    }

    #[test]
    fn example3() {
        let a = String::from("aaaaaa");
        let b = String::from("bbaaaa");
        let correct = false;
        let result = is_subsequence(a, b);

        assert_eq!(correct, result);
    }

    #[test]
    fn example4() {
        let a = String::from("twn");
        let b = String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxtxxxxxxxxxxxxxxxxxxxxwxxxxxxxxxxxxxxxxxxxxxxxxxn");
        let correct = true;
        let result = is_subsequence(a, b);

        assert_eq!(correct, result);
    }
}
