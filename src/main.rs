#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

#[derive(Debug)]
struct Square {
    x: f64,
    y: f64,
    side: f64
}

trait HasArea {
  fn area(&self) -> f64;

  fn is_larger(&self, &Self) -> bool;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
      std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger(&self, other: &Self) -> bool {
      self.area() > other.area()
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
      self.side * self.side
    }

    fn is_larger(&self, other: &Self) -> bool {
      self.area() > other.area()
    }
}







enum Option<T> {
  Some(T),
  None
}

enum Result<T, E> {
  Ok(T),
  Err(E)
}

fn main() {
  let x: Option<i32> = Option::Some(5);

  let q: Option<f64> = Option::Some(5f64);

  let a = 32;

  println!("{}", takes_anyhing::<i32>(a));


  let y = Circle {x: 1.0, y: 2.0, radius: 3.0};
  let k = Square {x: 1.0, y: 2.0, side: 3.0};
  let b = 10;
  print_area(y);
  print_area(k);
  // print_area(b); // would fail with 'trait not implemented'
}


fn takes_anyhing<T>(x: T) -> T {
  x
}


fn print_area<T: HasArea>(shape: T) {
  println!("{}", shape.area());
}