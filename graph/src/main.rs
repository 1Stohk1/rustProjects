use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 34,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });

    println!("There is a leaf in the graph {:?}", leaf);

    {
        let branch = Rc::new(Node {
            value: 14,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("Now the leaf has a parent {:?}", leaf.parent.borrow().upgrade());
    }

    println!("Now the leaf has lost the parent node {:?}", leaf.parent.borrow().upgrade());

}
