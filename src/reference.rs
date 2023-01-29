pub fn reference () {
  let mut x:i32 = 1;
  let ref_x: &mut i32 = &mut x;
  *ref_x = 666;
  println!("x的值为: {}",x)
}
