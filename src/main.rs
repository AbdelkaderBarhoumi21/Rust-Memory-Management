use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    println!("----- Memory Management Demo In Rust -----");
    //ownership
    let s1 = String::from("Ownership Example");
    let s2 = s1;
    println!("Ownership transferred to : {}", s2);
    // println!("{}", s1); // ❌ Error: s1 moved

    //borrowing
    let mut s3 = String::from("Borrowing example");
    borrow_demo(&s3);
    println!("After Borrowing : {} ", s3); // still accessible

    //mutable borrwing
    let mut s4 = String::from("Hello");
    mutate_demo(&mut s4);
    println!("After Mutation : {}", s4);
    //lifetimes
    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyz");
        result = longest(&a, &b);
        println!("Longest String : {}", result);
    }
    //box heap allocation
    let boxed = Box::new(42);
    println!("Boxed value : {}", boxed);
    //Rc (reference-counted pointer)

    let rc_value = Rc::new(String::from("Shared value"));
    let rc_clone = Rc::clone(&rc_value);
    println!("Rc values : {} - {}", rc_value, rc_clone);
    println!("Ref count : {}", Rc::strong_count(&rc_value));

    //RefCell(interior mutability)
    let cell = RefCell::new(100);
    *cell.borrow_mut() += 50;
    println!("Ref Cell Value Modified : {}", cell.borrow());
}
fn borrow_demo(data: &String) {
    println!("Borrowed : {}", data);
}
fn mutate_demo(data: &mut String) {
    data.push_str(" World");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
