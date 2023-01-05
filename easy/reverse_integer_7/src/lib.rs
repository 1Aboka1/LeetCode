fn reverse(x: i32) -> i32 {
    let mut s = x.to_string();
    s = s.chars().rev().collect::<String>();
    if s.chars().nth(0) == Some('-') {
        let a = &s[1..]; 
        let i: i32 = a.parse::<i32>().unwrap_or_else(|_| {
            return 0;
        });
        return -1 * i;
    }

    let i: i32 = s.parse::<i32>().unwrap_or_else(|_| {
        return 0;
    });

    i
}

#[cfg(test)]
mod tests {
    use crate::reverse;

    #[test]
    fn example1() {
        let x = 123;
        let answer = reverse(x);
        let correct_answer = 321;

        assert_eq!(correct_answer, answer);
    }

    #[test]
    fn example2() {
        let x = -123;
        let answer = reverse(x);
        let correct_answer = -321;

        assert_eq!(correct_answer, answer);
    }
}
