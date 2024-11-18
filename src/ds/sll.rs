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
        match self.head.as_mut() {
            Some(_) => {
                let mut node = self.head.as_mut();
                loop {
                    if let Some(nnode) = node{
                        match nnode.next.as_mut() {
                            Some(_) => {
                                node = nnode.next.as_mut();
                            }
                            None => {
                                nnode.next = Some(Box::new(MyLinkedListNode{value: val, next: None}));
                                return
                            }
                        }
                    }
                }
            }
            None => {
                self.head = Some(Box::new(MyLinkedListNode{value: val, next: None}));
            }
        }
    }
}