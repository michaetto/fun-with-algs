use std::fmt::Debug;

// Node of linked list.
#[derive(Debug)]
pub struct Node<V: Debug> {
    v: V,
    next: Option<Box<Node<V>>>,
}

fn reverse<V: Debug>(mut head: Option<Box<Node<V>>>) -> Option<Box<Node<V>>> {
    let mut node = head.take();
    let mut previous = None;

    while let Some(mut current) = node {
        let next = current.next.take(); // after this current.next is None
        current.next = previous.take();
        previous = Some(current);
        node = next;
    }

    previous
}

#[cfg(test)]
mod tests {
    use crate::data::linked_list::{Node, reverse};

    #[test]
    fn test_reverse() {
        // Create a new linked list
        let mut head = Some(Box::new(Node { v: 5, next: None }));
        head = Some(Box::new(Node { v: 4, next: head }));
        head = Some(Box::new(Node { v: 3, next: head }));
        head = Some(Box::new(Node { v: 2, next: head }));
        head = Some(Box::new(Node { v: 1, next: head }));

        let mut org_values = Vec::new();
        let mut node = &head;
        while let Some(current) = node {
            org_values.push(current.v);
            node = &current.next;
        }

        assert!(org_values == [1, 2, 3, 4, 5]);
        println!("Original Linked List: {:?}", head);
        
        // Reverse the linked list
        let reversed = reverse(head);
        let mut reversed_values = Vec::new();
        let mut node = &reversed;
        while let Some(current) = node {
            reversed_values.push(current.v);
            node = &current.next;
        }
        assert!(reversed_values == [5, 4, 3, 2, 1]);
        println!("Reversed Linked List: {:?}", reversed);
    }
}
