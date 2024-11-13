#[derive(Debug)]
pub struct SLL {
    head: Box::<Option<SLLNode>>
}

#[derive(Debug)]
struct SLLNode {
    value: usize,
    next: Box::<Option<SLLNode>>
}

impl SLL {
    pub fn new() -> Self {
        SLL{
            head: Box::new(None)
        }
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        let mut position: usize = 0;
        let mut node = self.head.as_ref();
        
        loop {
           match node {
            Some(n) => {
                if position == index{
                    return Some(n.value)
                }else{
                    node = n.next.as_ref();
                    position += 1;
                }
            }
            None => return None
           }
        }
    }

    pub fn insert(&mut self, value: usize) {
        if self.head.is_none() {
            self.head = Box::new(Some(SLLNode{value: value, next: Box::new(None)}));
            return
        }

        //let node = self.head.as_deref_mut();
        // match self.head.as_deref_mut() {
        //     Some(node) => {
        //         node.value
        //     }
        //     None => panic!("node is None")
        // } 
        
    }
}