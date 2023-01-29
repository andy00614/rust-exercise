struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    // 没有return后面不能加分号;
    // 如果后面加分号就得写return,好奇怪
   return self.width * self.height;
  }

  fn inc_width(&mut self, delta: u32) {
    self.width += delta;
  }
}

pub fn operate_rectangle() {
  let mut rect = Rectangle {width: 10, height: 5};
  println!("old area, {}", rect.area());
  rect.inc_width(20);
  println!("new area, {}", rect.area());
}
