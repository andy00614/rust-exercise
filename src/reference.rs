pub fn reference () {
  let mut x:i32 = 1;
  let ref_x: &mut i32 = &mut x;
  *ref_x = 666;
  println!("x的值为: {}",x)
}

// pub fn dangling_reference() {
//   let ref_x: &i32;
//   {
//     let x:i32 = 256;
//     ref_x = &x;
//   }
//   println!("ref_x: {ref_x}")
// }
