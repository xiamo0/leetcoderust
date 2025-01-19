pub fn test() {
    let head = Option::Some(Box::new(ListNode::new(1)));
    println!("{:?}", remove_nth_from_end(head, 1));
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut length: usize = 0;
    let mut x = &head;
    while let Some(node) = x.as_ref() {
        x = &node.next;
        length += 1;
    }
    let mut node = ListNode::new(0);
    node.next = head;
    let mut newHead = Option::Some(Box::new(node));
    let option = newHead;
    let i1 = length - n as usize + 1;
    for i in 0..i1 {
        let i2 = option?.val;
      

    }

    return newHead?.next;
}
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