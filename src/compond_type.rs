pub fn array_type() {
  let mut a: [i8; 10] = [42; 10];
  a[5] = 0;
  let x = 243i16;
  println!("a: {:?}", a);
  println!("{x}的16进制输出格式: {:x}", x);
  println!("{x}的2进制输出格式: {:b}", x);
}

pub fn tubple_print() {
  let t:(i8,bool) = (127, true);
  // 这里的 {} 是占位符
  println!("1st is {}", t.0);
  println!("2st is {}", t.1);
}
