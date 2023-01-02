// Definition for singly-linked list.
#![allow(dead_code, unused)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
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

pub struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut ans = None;
        let (mut i, mut i1, mut i2) = (&mut ans, &l1, &l2);

        while i1.is_some() || i2.is_some() {
            let mut sum = carry;
            if let Some(node) = i1 {
                sum += node.val;
                i1 = &node.next;
            }
            if let Some(node) = i2 {
                sum += node.val;
                i2 = &node.next;
            }
            carry = sum/10;
            *i = Some(Box::new(ListNode::new(sum%10)));
            i = &mut i.as_mut().unwrap().next;
        }
        //有进位产生，需要再加一位
        if carry == 1 {
            *i = Some(Box::new(ListNode::new(1)));
        }
        return ans;
    }
}

