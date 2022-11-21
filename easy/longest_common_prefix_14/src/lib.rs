use std::cmp::*;

fn compare_len_reverse_alpha(a: &String, b: &String) -> Ordering {
    let length_test = a.len().cmp(&b.len());
    if length_test == Ordering::Equal {
        return b.cmp(&a);
    }
    return length_test;
}

fn longest_common_prefix(mut strs: Vec<String>) -> String {
    strs.sort_unstable_by(compare_len_reverse_alpha);
    
    let mut ans: String = String::from("");
    let mut counter = 0;
    let smallest_str_len = strs[0].len();

    for i in (0..smallest_str_len + 1).rev() {
        for word in &strs {
            if &String::from(&word[..i]) == &strs[0][..i] {
                counter += 1;
            }
        }
        if counter == strs.len() {
            ans = String::from(&strs[0][..i]);
            break;
        }
        counter = 0;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        let result = longest_common_prefix(input);
        let correct_result = "fl";

        assert_eq!(correct_result, result);
    }

    #[test]
    fn example2() {
        let input = vec![String::from("dog"), String::from("racecar"), String::from("car")];
        let result = longest_common_prefix(input);
        let correct_result = "";

        assert_eq!(correct_result, result);
    }

    #[test]
    fn example3() {
        let input = vec![String::from("ab"), String::from("a")];
        let result = longest_common_prefix(input);
        let correct_result = "a";

        assert_eq!(correct_result, result);
    }

    #[test]
    fn example4() {
        let input = vec![String::from("reflower"), String::from("flow"), String::from("flight")];
        let result = longest_common_prefix(input);
        let correct_result = "";

        assert_eq!(correct_result, result);
    }
}
