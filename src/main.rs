mod btree;
use crate::btree::BTree;

fn main() {
    let mut my_btree = BTree::new(None);

    println!("Is empty? {:?}", my_btree.is_empty());

    my_btree.insert(10);
    my_btree.insert(11);
    my_btree.insert(1);
    my_btree.insert(3);

    println!("5 exists? {:?}", my_btree.find(5));
    println!("3 exists? {:?}", my_btree.find(3));

    println!("--Recorrido preorden--");
    my_btree.print_preorder();
    println!("--Recorrido en orden--");
    my_btree.print_inorder();
    println!("--Recorrido postorden--");
    my_btree.print_postorder(); 

    println!("Is Empty? {:?}", my_btree.is_empty())
}
