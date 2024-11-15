mod ds;

fn main() {
    let mut single_linked_list = ds::sll::MyLinkedList::new();

    single_linked_list.add_at_head(1);
    single_linked_list.add_at_head(0);
    
    //single_linked_list.add_at_tail(10);
    //single_linked_list.add_at_tail(20);

    //println!("{:?}", single_linked_list.get(2));
    println!("{:?}", single_linked_list);
}
