/*
https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/?envType=problem-list-v2&envId=2cktkvj
 */
pub fn test() {
    let s = String::from("cbaebabacd");
    let p = String::from("abc");
    println!("{:?}", find_anagrams(s, p))
}

// todo outtime
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut vec = Vec::new();
    if p.len() > s.len() {
        return vec;
    }
    for i in 0..s.len() {
        let mut interval: usize = 0;
        let mut p_copy = p.clone();
        for _ in 0..p.len() {
            let cur_index: usize = i + interval;
            interval = interval + 1;
            if cur_index >= s.len() {
                break;
            }
            let cur_char = s.chars().nth(cur_index).unwrap();
            let option = p_copy.find(cur_char);
            match option {
                Some(index) => {
                    p_copy.remove(index);
                }
                None => {}
            }
        }
        if p_copy.len()==0 {
            vec.push(i as i32);
        }
    }
    vec
}