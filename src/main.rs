mod ds;

fn main() {
    let mut single_linked_list = ds::sll::SLL::new();

    single_linked_list.insert(10);

    println!("{:?}", single_linked_list.get(1));
}
