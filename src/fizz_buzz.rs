fn is_devisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false
  }
  return lhs % rhs == 0
}

fn fizz_buzz(n: u32) {
  match(is_devisible_by(n,3),is_devisible_by(n,5)) {
    (true,true) => println!("fizzbuzz"),
    (true,false) => println!("fizz"),
    (false,true) => println!("buzz"),
    (false,false) => println!("{n}")
  }
}

pub fn fizz_buzz_to(n: u32) {
  for i in 1..=n {
    fizz_buzz(i)
  }
}

