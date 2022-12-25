use std::mem;

mod employee;
mod printinfo;
mod product;

fn main() {
    let mut v = vec![65, 122];
    let s = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };
    println!("{:?}", v);
    mem::forget(v);
    assert_eq!(s, "Az");
    mem::drop(s);
}
