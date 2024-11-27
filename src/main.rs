mod ds;

fn main() {
    let mut single_linked_list = ds::sll::MyLinkedList::new();

    single_linked_list.add_at_tail(10);
    single_linked_list.add_at_tail(20);

    println!("{:?}", single_linked_list.len());
    
    single_linked_list.add_at_head(9);
    single_linked_list.add_at_head(8);

    println!("{:?}", single_linked_list.len());

    single_linked_list.add_at_index(0, 6);
    single_linked_list.add_at_index(1, 7);
    

    println!("{:?}", single_linked_list.get(2));
    println!("{:?}", single_linked_list);
}
