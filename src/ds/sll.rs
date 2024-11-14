#[derive(Debug)]
pub struct MyLinkedList {
    head: Box::<Option<MyLinkedListNode>>
}

#[derive(Debug)]
struct MyLinkedListNode {
    value: i32,
    next: Box::<Option<MyLinkedListNode>>
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList{
            head: Box::new(None)
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
        let new_node = Box::new(Some(MyLinkedListNode{value: val, next: Box::new(None)}));
        
        if self.head.is_none() {
            self.head = new_node;
            return
        }
    }

    pub fn add_at_tail(&mut self, val: i32) {        
        if self.head.is_none() {
            self.head = Box::new(Some(MyLinkedListNode{value: val, next: Box::new(None)}));
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
                            node.next = Box::new(Some(MyLinkedListNode{value: val, next: Box::new(None)}));
                            return
                        }
                    }
                }
                None => panic!("Node is None")
            }
        }   
    }
}