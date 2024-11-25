/*
https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/?envType=problem-list-v2&envId=2cktkvj
 */
pub fn test() {
    let s = String::from("cbaebabacd");
    let p = String::from("abc");
    println!("{:?}", find_anagrams(s, p))
}

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut s_char_count: [i32; 26] = [0; 26];
    let mut p_char_count: [i32; 26] = [0; 26];
    let mut r = Vec::new();

    if s.len() < p.len() {
        return r;
    }
    let p_chars: Vec<char> = p.chars().collect();
    let s_chars: Vec<char> = s.chars().collect();

    let a_num = 'a' as usize;
    for i in 0..p.len() {
        let x = p_chars[i];
        let x1 = x as usize - a_num;
        p_char_count[x1] = p_char_count[x1] + 1;
        let y = s_chars[i];
        let y1 = y as usize - a_num;
        s_char_count[y1] = s_char_count[y1] + 1;
    }
    if s_char_count == p_char_count {
        r.push(0i32)
    }
    for i in 1..s.len() - p.len() + 1 {
        s_char_count = [0; 26];
        for j in 0..p.len() {
            let x = s_chars[i + j];
            let index = x as usize - a_num;
            s_char_count[index] = s_char_count[index] + 1;
        }
        if s_char_count == p_char_count {
            r.push(i as i32);
        }
    }
    return r;
}

pub fn find_anagrams2(s: String, p: String) -> Vec<i32> {
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
        if p_copy.len() == 0 {
            vec.push(i as i32);
        }
    }
    vec
}