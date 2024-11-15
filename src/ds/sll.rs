#[derive(Debug)]
pub struct MyLinkedList {
    head: Option<Box::<MyLinkedListNode>>
}

#[derive(Debug, Clone)]
struct MyLinkedListNode {
    value: i32,
    next: Option<Box::<MyLinkedListNode>>
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList{
            head: None
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut position: i32 = 0;
        let mut node = self.head.as_ref();
        
        loop {
           match node {
            Some(n) => {
                if position == index{
                    return n.value
                }else{
                    node = n.next.as_ref();
                    position += 1;
                }
            }
            None => return -1
           }
        }
    }

    pub fn add_at_head(&mut self, val: i32) {
        let next_node = self.head.clone();
        
        match &mut self.head {
            Some(node) => {  
                node.value = val;
                node.next = next_node;
            }
            None => {
                self.head = Some(Box::new(MyLinkedListNode{value: val, next: None}));
            }
        }
    }

    pub fn add_at_tail(&mut self, val: i32) {        
        if self.head.is_none() {
            self.head = Some(Box::new(MyLinkedListNode{value: val, next: None}));
            return
        }

        let mut opt_node = self.head.as_mut();

        loop {
            match opt_node {
                Some(node) => {
                    match node.next.as_ref() {
                        Some(_) => {
                            opt_node = node.next.as_mut();
                        }
                        None => {
                            node.next = Some(Box::new(MyLinkedListNode{value: val, next: None}));
                            return
                        }
                    }
                }
                None => panic!("Node is None")
            }
        }   
    }
}