
pub fn test() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let i = single_number(nums);
    println!("{}",i);
}
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut r = nums[0];
    for (index,value) in nums.iter().enumerate() {
        if index !=0 {
            r= r^(*value);
        }
    }
    return r;
}