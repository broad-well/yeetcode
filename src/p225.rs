use std::collections::VecDeque;

struct MyStack {
    queue_a: VecDeque<i32>,
    queue_b: VecDeque<i32>,
    using_a: bool
}

impl MyStack {
    
    fn new() -> Self {
        Self { queue_a: VecDeque::new(), queue_b: VecDeque::new(), using_a: true }
    }
    
    fn push(&mut self, x: i32) {
        // clear the inactive queue (it should hold at most 1 element)
        if let Some(elem) = self.inactive_queue().pop_front() {
            self.active_queue().push_back(elem);
        }
        self.inactive_queue().push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.inactive_queue().is_empty() {
            self.shift_elements();
        }
        assert!(self.inactive_queue().len() == 1);
        self.inactive_queue().pop_front().unwrap()
    }
    
    // mut because internal rearrangement might be necessary to provide this answer
    fn top(&mut self) -> i32 {
        if self.inactive_queue().is_empty() {
            self.shift_elements();
        }
        assert!(self.inactive_queue().len() == 1);
        *self.inactive_queue().front().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.queue_a.is_empty() && self.queue_b.is_empty()
    }

    #[inline]
    fn active_queue(&mut self) -> &mut VecDeque<i32> {
        if self.using_a {
            &mut self.queue_a
        } else {
            &mut self.queue_b
        }
    }

    #[inline]
    fn inactive_queue(&mut self) -> &mut VecDeque<i32> {
        if self.using_a {
            &mut self.queue_b
        } else {
            &mut self.queue_a
        }
    }

    fn shift_elements(&mut self) {
        while self.active_queue().len() > 1 {
            let elem = self.active_queue().pop_front().unwrap();
            self.inactive_queue().push_back(elem);
        }
        // The one remaining item in the active queue is the one to pop
        self.using_a ^= true; // fancy negation
    }
}

/*
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_behavior() {
        let mut stack = MyStack::new();
        stack.push(2);
        stack.push(7);
        stack.push(10);
        assert_eq!(stack.top(), 10);
        assert_eq!(stack.pop(), 10);
        assert_eq!(stack.top(), 7);
        assert!(!stack.empty());
        assert_eq!(stack.pop(), 7);
        stack.push(8);
        assert_eq!(stack.pop(), 8);
        assert_eq!(stack.pop(), 2);
        assert!(stack.empty());
    }

    #[test]
    fn failed_testcase() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        stack.push(3);
        assert_eq!(stack.top(), 3);
    }
}