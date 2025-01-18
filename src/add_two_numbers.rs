use std::cmp::max;

pub fn test() {
    let mut node1 = ListNode::new(9);
    // let mut node11 = ListNode::new(4);
    // let mut node111 = ListNode::new(9);
    // node11.next = Option::Some(Box::new(node111));
    // node1.next = Option::Some(Box::new(node11));
    let list1 = Option::Some(Box::new(node1));

    let mut node2 = ListNode::new(1);
    let mut node22 = ListNode::new(9);
    let mut node222 = ListNode::new(9);
    let mut node2222 = ListNode::new(9);
    node222.next = Option::Some(Box::new(node2222));
    node22.next = Option::Some(Box::new(node222));
    node2.next = Option::Some(Box::new(node22));
    let list2 = Option::Some(Box::new(node2));

    // println!("{:?}", add_two_numbers(list1, list2));
    println!("{:?}", add_two_numbers_2(list1, list2));
}
/**
输入：l1 = [2,4,3], l2 = [5,6,4]
输出：[7,0,8]
解释：342 + 465 = 807.

[2,4,9] 942
[5,6,4,9] 9465
[7,0,4,0,1] 10407
2147483647
9223372036854775807
*/
pub fn add_two_numbers_2(l1: Option<Box<crate::add_two_numbers::ListNode>>, l2: Option<Box<crate::add_two_numbers::ListNode>>) -> Option<Box<crate::add_two_numbers::ListNode>> {
    let mut current_num1 = &l1;
    let mut vector1: Vec<i32> = Vec::new();
    while let Some(n1) = current_num1.as_ref() {
        vector1.push(n1.val);
        current_num1 = &n1.next;
    }

    let mut current_num2 = &l2;
    let mut vector2: Vec<i32> = Vec::new();
    while let Some(n2) = current_num2.as_ref() {
        vector2.push(n2.val);
        current_num2 = &n2.next;
    }
    let mut vector3: Vec<i32> = Vec::new();
    let len1 = vector1.len();
    let len2 = vector2.len();
    let maxLen = max(len1, len2);
    let mut sum = 0;
    let mut remaining = 0;
    for i in 0..maxLen {
        let option1 = vector1.get(i);
        let option2 = vector2.get(i);

        if option1.is_some() {
            sum = vector1[i];
        }
        if option2.is_some() {
            sum += vector2[i];
        }
        sum += remaining;

        if sum < 10 {
            vector3.push(sum);
            remaining = 0;
            sum = 0;
            continue;
        }
        if sum >= 10 {
            vector3.push(sum % 10);
            remaining = sum / 10;
            sum = 0;
            continue;
        }
    }
    if remaining != 0 {
        vector3.push(remaining);
    }
    let mut result_box: Option<Box<ListNode>> = None;
    for x in vector3 {
        if result_box == None {
            result_box = Some(Box::new(ListNode::new(x)));
        } else {
            let mut current_box = &mut result_box;
            while let Some(n2) = current_box.as_mut() {
                if n2.next == None {
                    n2.next = Some(Box::new(ListNode::new(x)));
                    break;
                } else {
                    current_box = &mut n2.next;
                }
            }
        }
    }

    if result_box == None {
        result_box = Option::from(Box::new(ListNode::new(0)));
    }
    result_box
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut num1 = 0;
    let mut num2 = 0;
    let mut current_num1 = &l1;
    let mut loopNum = 0;
    while let Some(n1) = current_num1.as_ref() {
        let mut cur = n1.val;
        for i in 0..loopNum {
            cur = 10 * cur;
        }
        num1 = num1 + cur;
        current_num1 = &n1.next;
        loopNum += 1;
    }
    let mut current_num2 = &l2;
    loopNum = 0;
    while let Some(n2) = current_num2.as_ref() {
        let mut cur = n2.val;
        for i in 0..loopNum {
            cur = 10 * cur;
        }
        num2 = num2 + cur;
        current_num2 = &n2.next;
        loopNum += 1;
    }

    let mut sum = num1 + num2;
    let mut result_box: Option<Box<ListNode>> = None;
    while sum != 0 {
        let i = sum % 10;
        sum = sum / 10;
        if result_box == None {
            result_box = Some(Box::new(ListNode::new(i)));
        } else {
            let mut current_box = &mut result_box;
            while let Some(n2) = current_box.as_mut() {
                if n2.next == None {
                    n2.next = Some(Box::new(ListNode::new(i)));
                    break;
                } else {
                    current_box = &mut n2.next;
                }
            }
        }
    }
    if result_box == None {
        result_box = Option::from(Box::new(ListNode::new(0)));
    }
    result_box
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}