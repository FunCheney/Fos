use std::cell::Cell;

fn main() {
    let v = Cell::new(vec![1,2,3]);
    f(&v);
    assert_eq!(v.into_inner(), vec![1,2,3,5]);
}

fn f(v: &Cell<Vec<i32>>){
    // Replaces the contents of the Cell with an empty Vec
    let mut v2 = v.take();
    println!("v2 {:?}", v2);
    v2.push(5);
    // // Put the modified Vec back
    v.set(v2)
}