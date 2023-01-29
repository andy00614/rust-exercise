pub fn slice() {
    let a: [i32; 5] = [1, 5, 8, 12, 22];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
    println!("【after slice】 a.3 is: {}, a is: {:?}",a[3], a);
}
