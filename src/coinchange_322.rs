use std::vec;

/**
 * https://leetcode.cn/problems/coin-change/description/?envType=problem-list-v2&envId=2cktkvj
 */
pub fn test() {
    let coins: Vec<i32> = vec![1, 2, 3, 4];
    let amount = 23;
    print!("r {}", coin_change(coins, amount));
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 { return 0; }
    if amount < 0 { return -1; }
    if coins.is_empty() { return -1; }
    let mut dp: Vec<i32> = vec![0; (amount + 1) as usize];
    let i1 = coins.len();
    for i in 1..amount + 1 {
        dp[i as usize] = amount + 1;
        for j in 0..i1 {
            let x = *(coins.get(j).unwrap());
            if i - x >= 0 {
                let temp = dp[(i - x) as usize] + 1;
                if dp[i as usize] > temp {
                    dp[i as usize] = temp;
                }
            }
        }
    }
    let min = if dp[amount as usize] != (amount + 1) { dp[amount as usize] } else { -1 };
    return min;
}