use std::f64;

struct Circle
{
  x: f64,
  y: f64,
  radius: f64
}

impl Circle {
  fn new(x: f64, y: f64, radius: f64) -> Circle {
    Circle {x: x, y: y, radius: radius}
  }
}

impl Circle {
  fn area(&self) -> f64 {
    // let mut area =
    std::f64::consts::PI * (self.radius * self.radius)
    // Circle {area: area, .. self}
  }
}

impl Circle {
  fn dist_from_origin(&self) -> f64 {
    ( (self.x * self.x) + (self.y + self.y) ).sqrt()
  }
}

impl Circle {
  fn grow(&self, increment: f64) -> Circle {
    Circle {x: self.x, y: self.y, radius: self.radius + increment}
  }
}


struct CircleBuilder
{
  x: f64,
  y: f64,
  radius: f64
}

impl CircleBuilder {
  fn new() -> CircleBuilder {
    CircleBuilder {x: 0.0, y: 0.0, radius: 1.0}
  }

  fn x(&mut self, x: f64) -> &mut CircleBuilder {
    self.x = x;
    self
  }

  fn y(&mut self, y: f64) -> &mut CircleBuilder {
    self.y = y;
    self
  }

  fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
    self.radius = radius;
    self
  }

  fn finalize(&self) -> Circle {
    Circle {x: self.x, y: self.y, radius: self.radius}
  }
}

fn main()
{
  let c = Circle::new(2.0, 3.0, 2.0);
  println!("{}", c.area());
  println!("{}", c.grow(10.0).area());

  let c2 = CircleBuilder::new()
    .x(1.0)
    .y(2.0)
    .radius(2.0)
    .finalize();

  println!("{}", c2.area());



  let gr = "Hello \
    there"; // &'static str
  println!("{}", gr);

  let mut st = gr.to_string();
  st.push_str(" world");
  println!("{}", st);

  let hachiko = "忠犬ハチ公";

  for b in hachiko.as_bytes() {
      print!("{}, ", b);
  }

  println!("\n{:?}", hachiko.chars().nth(3));

  let y = "Hello, ".to_string();
  let y2 = " world".to_string();

  println!("{}", y + &y2);
}