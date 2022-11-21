fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len()];

    for i in 1..nums.len() + 1 {
        println!("{}", &nums[..i].iter().sum::<i32>());
        ans[i - 1] = nums[..i].iter().sum();
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, 2, 3, 4];
        let result = running_sum(input);
        let correct_result = vec![1, 3, 6, 10];

        assert_eq!(correct_result, result);
    }

    #[test]
    fn example2() {
        let input = vec![1,1,1,1,1];
        let result = running_sum(input);
        let correct_result = vec![1, 2, 3, 4, 5];

        assert_eq!(correct_result, result);
    }

    #[test]
    fn example3() {
        let input = vec![3,1,2,10,1];
        let result = running_sum(input);
        let correct_result = vec![3, 4, 6, 16, 17];

        assert_eq!(correct_result, result);
    }

}
