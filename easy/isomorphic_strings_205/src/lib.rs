fn is_isomorphic(s: String, t: String) -> bool {
    for i in 0..s.len() {
        let v: Vec<_> = s.match_indices(s.as_bytes()[i] as char).collect();
        let v2: Vec<_> = t.match_indices(t.as_bytes()[i] as char).collect();

        if v.len() != v2.len() {
            return false;
        }
        for i in 0..v.len() {
            if v[i].0 == v2[i].0 {

            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = String::from("egg");
        let t = String::from("add");
        let correct = true;
        let result = is_isomorphic(s, t);

        assert_eq!(correct, result);
    }

    #[test]
    fn example2() {
        let s = String::from("foo");
        let t = String::from("bar");
        let correct = false;
        let result = is_isomorphic(s, t);

        assert_eq!(correct, result);
    }

    #[test]
    fn example3() {
        let s = String::from("paper");
        let t = String::from("title");
        let correct = true;
        let result = is_isomorphic(s, t);

        assert_eq!(correct, result);
    }
}
