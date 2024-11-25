/*
https://leetcode.cn/problems/find-all-numbers-disappeared-in-an-array/description/?envType=problem-list-v2&envId=2cktkvj
 */

pub fn test() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 7, 9, 10];
    println!("r {:?}", find_disappeared_numbers(nums))
}


pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    println!("{}", len);
    let mut r = vec![0; len];
    for (_, value) in nums.iter().enumerate() {
        r[(*value -1) as usize] = 1;
    }
    let mut result = vec![];
    for (index, value) in r.iter().enumerate() {
        if *value == 0 {
            result.push((index+1) as i32);
        }
    }
    result
}