#[derive(Debug)]
pub struct MyLinkedList {
    head: Option<Box::<MyLinkedListNode>>,
    len: i32
}

#[derive(Debug, Clone)]
struct MyLinkedListNode {
    value: i32,
    next: Option<Box::<MyLinkedListNode>>
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList{
            head: None,
            len: 0
        }
    }

    pub fn len(&self) -> i32 {
        self.len
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
        let new_node = Box::new(
            MyLinkedListNode{
                value: val,
                next: self.head.take()
            }
        );
        self.head = Some(new_node);
        self.len= self.len+1;
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
                                self.len = self.len+1;
                                return
                            }
                        }
                    }
                }
            }
            None => {
                self.head = Some(Box::new(MyLinkedListNode{value: val, next: None}));
                self.len = self.len+1;
            }
        }
    }

    pub fn add_at_index(&mut self, index: i32, val: i32){        
        if index == 0 {
            self.add_at_head(val);
            return
        }

        if index == self.len {
            self.add_at_tail(val);
            return
        }

        let mut i = 0;
        self.head.as_mut().map(|mut node|{
            loop {
                if i + 1 == index {
                    let new_node = Box::new(
                        MyLinkedListNode{
                            value: val,
                            next: node.next.take()
                        }
                    );
                    node.next = Some(new_node);
                    self.len = self.len + 1;
                    return
                }
                match node.next.as_mut() {
                    Some(next_node) => {
                        node = next_node;
                        i = i + 1;
                    }
                    None => {return}
                }
            }
        });
    }
}