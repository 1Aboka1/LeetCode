fn pivot_index(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    
    for i in 0..nums_len {
        if nums[..i].iter().sum::<i32>() == nums[i + 1..nums_len].iter().sum::<i32>() {
            return i as i32;
        }
    }

    -1
}
