mod compond_type;
mod reference;
mod slice;
mod fizz_buzz;
mod method;
fn base() {
    // let mut x: i32 = 256;
    let mut x = 256i16;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = x * 3 + 1
        }
        print!("-> {x}")
    }
    println!()
}
fn main() {
   base();
   compond_type::array_type();
   compond_type::tubple_print();
   reference:: reference();
   slice::slice();
   fizz_buzz::fizz_buzz_to(20);
   method:: operate_rectangle();
}
