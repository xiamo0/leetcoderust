//https://leetcode.cn/problems/palindromic-substrings/description/?envType=problem-list-v2&envId=2cktkvj
pub fn test() {
    let s = String::from("abc");
    println!("{}", count_substrings(s));
}

pub fn count_substrings(s: String) -> i32 {
    fn isPalindromicString(s: String) -> bool {
        let len = s.len();

        let chars :Vec<char> = s.chars().collect();

        for i in 0..(len / 2) {
            let x =chars[i];
            let x1 = chars[len - 1 - i];

            if x != x1 {
                return false;
            }
        }
        return true;
    }

    let len = s.len();
    let mut count = 0;
    for endIndex in 0..len + 1 {
        for beginIndex in 0..endIndex {
            if isPalindromicString(s[beginIndex..endIndex].to_string()) {
                count += 1
            }
        }
    }
    return count;
}


pub fn count_substrings2(s: String) -> i32 {
    let len = s.len();
    let mut count = 0;
    for endIndex in 0..len {
        for beginIndex in 0..endIndex {
            if isPalindromicString(s[beginIndex..endIndex].to_string()) {
                count += 1
            }
        }
    }
    return count;
}

pub fn isPalindromicString(s: String) -> bool {
    let len = s.len();

    for i in 0..(len / 2) {
        if s.chars().nth(i).unwrap() == s.chars().nth(len - 1 - i).unwrap() {
            return false;
        }
    }

    return true;
}