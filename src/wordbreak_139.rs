//https://leetcode.cn/problems/word-break/description/?envType=problem-list-v2&envId=2cktkvj
pub fn test() {
    let s = String::from("leetcode");
    let word = vec![String::from("leet"), String::from("code")];
    let r = word_break(s, word);
    println!("flag : {}", r);
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let len = s.len() + 1;
    let mut flags: Vec<u8> = vec![0; len];
    flags[0] = 1;
    for i in 1..len {
        for j in 0..i {
            let substring = &s[j..i];
            let x = word_dict.contains(&String::from(substring));

            let x1 = flags[j] == 1;
            if x && x1 {
                flags[i] = 1;
            }
        }
    }

    return flags[s.len()] == 1;
}