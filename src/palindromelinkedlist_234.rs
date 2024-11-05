
pub fn test() {
    let mut n1=ListNode::new(1);
    let n2=ListNode{val:12,next:None};
    n1.next=Some(Box::new(n2));
    let s=Solution{};
    let flag= s.is_palindrome(Some(Box::new(n1)));
    println!("flag : {}",flag);

}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution {
}
impl Solution {
    pub fn is_palindrome(&self,head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let mut vec=vec![];
        let mut n1=head;

        while n1.is_some(){
            let temp=n1.unwrap();
            vec.push(temp.val);
            n1=temp.next;
        }
        let len =vec.len();
        println!(" vec length {}", len);
        for i in 0..len/2{
            if vec[i]!=vec[len-i-1]{
                return false;
            }
        }
        return true;


    }
}