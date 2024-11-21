use crate::coinchange_322::coin_change;

/**
 * https://leetcode.cn/problems/hamming-distance/description/?envType=problem-list-v2&envId=2cktkvj
 */

pub fn test() {
    let x=4;
    let y=1;
    print!("r {}", hamming_distance(x, y));
}

pub fn hamming_distance2(x: i32, y: i32) -> i32 {
    return (x^y).count_ones() as i32;
}
pub fn hamming_distance(x: i32, y: i32) -> i32 {

    let z=x^y;
    let zstr =format!("{:b}", z);
    let mut  count=0;
    for x in zstr.chars() {
        if x=='1' {
            count=count+1;
        }
    }
    return count;

}
