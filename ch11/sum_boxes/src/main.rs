use std::ops::Add;
fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);
    // println!("one {}", &*one);
    // println!("two {}", &*two);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);
    println!("Test Passed!");
}

fn sum_boxes<T: Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    let c = *a + *b;
    Box::new(c)
}
