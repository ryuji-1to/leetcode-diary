use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Test {
    id: i32,
}

struct Test2 {
    id: Cell<i32>,
    id2: i32,
}

pub fn run() {
    let a = Rc::new(RefCell::new(Test { id: 1 }));
    let a1 = a.clone();
    let b1 = Rc::downgrade(&a);

    println!(
        "strong  is {}, weak is {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    if let Some(strong_ref) = b1.upgrade() {
        println!("{}", strong_ref.borrow().id);
    }

    let rc = RefCell::new(10);
    *rc.borrow_mut() = 11;
    println!("{}", rc.borrow());

    let t2 = Test2 {
        id: Cell::new(10),
        id2: 0,
    };
    println!("cell {}", t2.id.get());
    t2.id.set(11);
    // t2.id2 = 10;
    println!("cell {}", t2.id.get());
}
