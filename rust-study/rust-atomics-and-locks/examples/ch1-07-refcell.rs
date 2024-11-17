use std::cell::{RefCell};
fn main() {
    let v = RefCell::new(vec![1,2,3]);
    f(&v);
    assert_eq!(v.into_inner(),vec![1,2,3,1]);

}

fn f(v: &RefCell<Vec<i32>>) {
    // 这里可直接修改 vec
    v.borrow_mut().push(1);
}