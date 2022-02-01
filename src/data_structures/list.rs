// #[derive(Debug)]
// struct LinkedNode<'a> {
//     value: i32,
//     next: Option<&'a LinkedNode<'a>>
// }

// impl<'a> LinkedNode<'a> {
//     fn new(value: i32) -> Self {
//         LinkedNode {
//             value,
//             next: None
//         }
//     }
// }

// #[derive(Debug)]
// struct LinkedList<'link> {
//     head: Option<'link, LinkedNode<'link>>
// }

// impl<'link> LinkedList<'link> {
//     fn new(value: i32) -> Self {
//         Self {
//             head: Some(&LinkedNode::new(value))
//         }
//     }

//     fn insert_at_head(&mut self, value: i32) {
//         let mut node = LinkedNode::new(value);

//         if let Some(head)  = self.head{
//             node.next = Some(head)
//         } else {
//             self.head = Some(&node);
//         }
//     }
// }