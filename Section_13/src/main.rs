use std::cell::RefCell;
use std::rc::Rc;

struct Flagger {
    is_true: RefCell<bool>,
}

fn main() {
    let t = (12, "eggs"); // create on the stack
    let b = Box::new(t); // create on the heap, but b was stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    println!("{}, {}, {}", s1.contains("Point"), s2, s3.contains("ter"));

    let flag = Flagger {
        is_true: RefCell::new(true),
    };

    // borrow retunr Ref<T>
    // borrow_mut return RefMut<T>

    // let reference = flag.is_true.borrow();
    // println!("{}", reference);

    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false; // derefence first to access inside
    println!("{}", mut_ref);
}
