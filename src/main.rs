// use std::io;

fn main() {
  let mut degree = 32.0;
  while degree <= 212.0 {
    let celsius = fahrenheit_to_celsius(degree);
    println!("{degree} degrees in fahrenheiht is {celsius} degrees in celsius");
    degree += 1.0;
  }
}

fn fahrenheit_to_celsius(deg: f64) -> f64 {
  ((deg - 32.0) * 5.0) / 9.0
}
