use std::collections::HashSet;

//https://leetcode.cn/problems/longest-consecutive-sequence/description/?envType=problem-list-v2&envId=2cktkvj
pub fn test() {
    let v = vec![1, 2, 4];
    println!("{}", longest_consecutive(v));
}
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set:HashSet<i32>  = HashSet::new();
    for x in nums {
        set.insert(x);
    }

    let set1 = set.clone();
    let mut result =0;
    for mut x in set1 {
        let mut curLongest=0;
        while set.contains(&x) {
            curLongest+=1;
            x=x+1;
        }
        if curLongest > result {
            result=curLongest;
        }
        
    }

    return result;
    
}