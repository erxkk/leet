// https://leetcode.com/problems/add-two-numbers/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    // ignore empty slices for simplicity
    #[inline]
    fn from_slice(slice: &[i32]) -> Box<Self> {
        if slice.len() == 0 {
            unreachable!();
        }

        Box::new(Self {
            val: slice[0],
            next: slice
                .into_iter()
                .skip(1)
                .rev()
                .map(|val| Box::new(Self::new(*val)))
                .reduce(|next, mut curr| {
                    curr.next = Some(next);
                    curr
                }),
        })
    }

    // used for simple assertion
    #[inline]
    fn into_vec(self) -> Vec<i32> {
        let mut res = vec![self.val];

        let mut curr = self;
        while let Some(next) = curr.next {
            res.push(next.val);
            curr = Box::into_inner(next);
        }

        res
    }

    // unused
    #[inline]
    fn append(&mut self, val: i32) {
        let mut curr = self;
        while let Some(ref mut next) = curr.next {
            curr = &mut *next;
        }

        curr.next = Some(Box::new(Self::new(val)));
    }
}

// time: O(2n) n > m
// mem: O(2n) n > m
// given 2 linked lists of single digit numbers per node with not trailing 0s
// (representing a number in reverse order) create a new linked list that is
// the sum of both
pub fn add_two_numbers(l1: &Box<ListNode>, l2: &Box<ListNode>) -> Box<ListNode> {
    let mut carry = 0;
    let mut curr_l = Some(l1);
    let mut curr_r = Some(l2);

    // TODO: build from head first without vec
    // it seems that there's no easy way to build a linked list head first
    let mut nodes = vec![];

    loop {
        match (curr_l, curr_r) {
            (Some(l), Some(r)) => {
                curr_l = l.next.as_ref();
                curr_r = r.next.as_ref();
                nodes.push((l.val + r.val + carry) % 10);
                carry = (l.val + r.val + carry) / 10;
            }
            (None, Some(r)) => {
                curr_l = None;
                curr_r = r.next.as_ref();
                nodes.push((r.val + carry) % 10);
                carry = (r.val + carry) / 10;
            }
            (Some(l), None) => {
                curr_l = l.next.as_ref();
                curr_r = None;
                nodes.push((l.val + carry) % 10);
                carry = (l.val + carry) / 10;
            }
            (None, None) => {
                if (1..10).contains(&carry) {
                    nodes.push(carry);
                }
                break;
            }
        };
    }

    ListNode::from_slice(nodes.as_slice())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_carry() {
        let l1 = ListNode::from_slice(&[2, 4, 3]);
        let l2 = ListNode::from_slice(&[5, 6, 4]);
        let res = vec![7, 0, 8];
        assert_eq!(add_two_numbers(&l1, &l2).into_vec(), res);
    }

    #[test]
    fn zero() {
        let l1 = ListNode::from_slice(&[0]);
        let l2 = ListNode::from_slice(&[0]);
        let res = vec![0];
        assert_eq!(add_two_numbers(&l1, &l2).into_vec(), res);
    }

    #[test]
    fn carry_longer() {
        let l1 = ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_slice(&[9, 9, 9, 9]);
        let res = vec![8, 9, 9, 9, 0, 0, 0, 1];
        assert_eq!(add_two_numbers(&l1, &l2).into_vec(), res);
    }
}
