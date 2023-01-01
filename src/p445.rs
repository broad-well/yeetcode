// Definition for singly-linked list.
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

struct Solution;
// Copy the following

type List = Option<Box<ListNode>>;

// Tail recursive
fn list_len(list: &List, len: i8) -> i8 {
    if let Some(node_box) = list {
        list_len(&node_box.next, 1 + len)
    } else {
        len
    }
}

// Tail recursive
fn list_skip(list: &List, count: i8) -> &List {
    if count == 0 {
        list
    } else {
        list_skip(&list.as_ref().unwrap().next, count - 1)
    }
}

fn aligned_add(l1: &List, l2: &List) -> (Option<Box<ListNode>>, bool) {
    if let (Some(val1), Some(val2)) = (l1, l2) {
        let (accum, carry) = aligned_add(&val1.next, &val2.next);
        let sum = val1.val + val2.val + if carry { 1 } else { 0 };
        let node = ListNode {
            val: sum % 10,
            next: accum
        };
        (Some(Box::new(node)), sum >= 10)
    } else {
        (None, false)
    }
}

// I would also return a pointer to the last node if I could, but how do we return (moved object, borrowed part of that moved object, ...)?
fn carry_and_copy(longer_list: &List, skip_count: i8, carry: bool, existing_list: List) -> (List, bool) {
    if skip_count == 0 {
        // sublist is empty, so if carry, just return [1]
        if carry {
            let mut node = Box::new(ListNode::new(1));
            node.next = existing_list;
            (Some(node), false)
        } else {
            (existing_list, false)
        }
    } else if skip_count == 1 {
        let last_digit = longer_list.as_ref().unwrap();
        let new_digit = last_digit.val + if carry { 1 } else { 0 };
        let mut new_node = Box::new(ListNode::new(new_digit % 10));
        new_node.next = existing_list;
        (Some(new_node), new_digit >= 10)
    } else {
        let first = longer_list.as_ref().unwrap();
        let (rest_start, carry_in) = carry_and_copy(&first.next, skip_count - 1, carry, existing_list);
        let first_new = first.val + if carry_in { 1 } else { 0 };
        let first_node = ListNode {
            val: first_new % 10,
            next: rest_start
        };
        (Some(Box::new(first_node)), first_new >= 10)
    }
}

impl Solution {

    pub fn add_two_numbers(l1: List, l2: List) -> List {
        let len1 = list_len(&l1, 0);
        let len2 = list_len(&l2, 0);
        let skip_count = (len1 - len2).abs();
        let (sl1, sl2) = if len1 > len2 {
            (list_skip(&l1, skip_count), &l2)
        } else {
            (&l1, list_skip(&l2, skip_count))
        };

        let (aligned_sum, aligned_carry) = aligned_add(sl1, sl2);
        let (head_start, head_needs_1) = carry_and_copy(if len1 > len2 {
            &l1
        } else {
            &l2
        }, skip_count, aligned_carry, aligned_sum);
        
        let actual_head = if head_needs_1 {
            Some(Box::new(ListNode {
                val: 1,
                next: head_start
            }))
        } else { head_start };

        actual_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;

    fn build_list(mut list: impl Iterator<Item = i32>) -> Option<Box<ListNode>> {
        if let Some(val) = list.next() {
            let mut node = ListNode::new(val);
            node.next = build_list(list);
            Some(Box::new(node))
        } else {
            None
        }
    }

    fn collect_list(node: Option<Box<ListNode>>) -> LinkedList<i32> {
        if let Some(the_box) = node {
            let mut sublist = collect_list(the_box.next);
            sublist.push_front(the_box.val);
            sublist
        } else {
            LinkedList::new()
        }
    }

    #[test]
    fn samples() {
        let list1 = build_list(vec![7, 2, 4, 3].into_iter());
        let list2 = build_list(vec![5, 6, 4].into_iter());
        let result = collect_list(Solution::add_two_numbers(list1, list2));
        assert_eq!(result.into_iter().collect::<Vec<_>>(), vec![7, 8, 0, 7]);
    }

    #[test]
    fn failed_testcase() {
        let list1 = build_list(vec![3,9,9,9,9,9,9,9,9,9].into_iter());
        let list2 = build_list(vec![4].into_iter());
        let result = collect_list(Solution::add_two_numbers(list1, list2));
        assert_eq!(result.into_iter().collect::<Vec<_>>(), vec![4,0,0,0,0,0,0,0,0,3]);
    }
}