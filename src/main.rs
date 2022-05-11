#[derive(PartialEq, Eq, Clone, Debug)]

struct Answer;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node { next: None, val }
    }
}

type List = Option<Box<Node>>;

impl Answer {
    pub fn reverse_list(mut head: List) -> List {
        let mut new_head = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = new_head;
            new_head = Some(node);
        }
        new_head
    }

    pub fn add_two_numbers(mut l1: List, mut l2: List) -> List {
        let mut dummy = Node::new(0);
        let mut ptr = &mut dummy;
        let mut carry = 0;

        l1 = Answer::reverse_list(l1);
        l2 = Answer::reverse_list(l2);

        while (l1.is_some() || l2.is_some()) || carry > 0 {
            let mut num1 = 0;
            let mut num2 = 0;

            if let Some(v) = l1 {
                num1 = v.val;
                l1 = v.next;
            };
            if let Some(v) = l2 {
                num2 = v.val;
                l2 = v.next;
            };

            let mut sum = num1 + num2 + carry;
            carry = sum / 10;
            sum = sum % 10;

            ptr.next = Some(Box::new(Node::new(sum)));
            ptr = ptr.next.as_mut().unwrap();
        }
        Answer::reverse_list(dummy.next)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input1() {
        let l1 = Some(Box::new(Node {
            val: 7,
            next: Some(Box::new(Node {
                val: 2,
                next: Some(Box::new(Node {
                    val: 4,
                    next: Some(Box::new(Node::new(3))),
                })),
            })),
        }));
        let l2 = Some(Box::new(Node {
            val: 5,
            next: Some(Box::new(Node {
                val: 6,
                next: Some(Box::new(Node { val: 4, next: None })),
            })),
        }));
        let expected = Some(Box::new(Node {
            val: 7,
            next: Some(Box::new(Node {
                val: 8,
                next: Some(Box::new(Node {
                    val: 0,
                    next: Some(Box::new(Node::new(7))),
                })),
            })),
        }));
        assert_eq!(Answer::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_input2() {
        let l1 = Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node {
                val: 4,
                next: Some(Box::new(Node::new(3))),
            })),
        }));
        let l2 = Some(Box::new(Node {
            val: 5,
            next: Some(Box::new(Node {
                val: 6,
                next: Some(Box::new(Node { val: 4, next: None })),
            })),
        }));
        let expected = Some(Box::new(Node {
            val: 8,
            next: Some(Box::new(Node {
                val: 0,
                next: Some(Box::new(Node::new(7))),
            })),
        }));
        assert_eq!(Answer::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_input3() {
        let l1 = Some(Box::new(Node { val: 0, next: None }));
        let l2 = Some(Box::new(Node { val: 0, next: None }));
        let expected = Some(Box::new(Node { val: 0, next: None }));
        assert_eq!(Answer::add_two_numbers(l1, l2), expected);
    }
}
