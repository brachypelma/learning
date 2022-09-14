// use std::io;

fn main() {
  for num in (1..10) {
    let fib = nth_fibonacci(num);
    println!("The {num}th fibonacci number is {fib}");
  }
}

fn nth_fibonacci(num: i32) -> i32 {
  let mut a = 0;
  let mut b = 1;
  let mut c = a + b;
  let mut counter = 0;

  let result = loop {
    c = a + b;
    a = b;
    b = c;
    counter += 1;

    if counter == num {
      break c;
    }
  };

  return result;
}
