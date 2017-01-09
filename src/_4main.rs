use std::cell::RefCell;

struct Point
{
  x: i32,
  y: i32
}

struct Point3d
{
  x: i32,
  y: i32,
  z: i32
}

struct PointRef<'a>
{
  x: &'a mut i32,
  y: &'a mut i32
}

enum Message
{
  Quit,
  ChangeColor(u32, u32, u32),
  Write(String),
  Move { x: i32, y: i32 },
}

fn main()
{
  let mut x = 5;
  println!("{}", x);

  x = 6;
  println!("{}", x);

  let y = &mut x;
  // let k = 10;
  // y = &k;
  *y += 1;
  println!("{}", y);


  let x = RefCell::new(42);

  let y = x.borrow_mut();
  // let z = x.borrow_mut();

  let mut origin = Point { x:0, y: 0 };
  origin.x = 5;
  println!("{}, {}", origin.x, origin.y);

  let base_point = Point3d { x: 3, y: 8, z: 0 };
  let point = Point3d { z: 10, .. base_point };
  println!("{}, {}, {}", point.x, point.y, point.z);

  {
    let r = PointRef { x: &mut 10, y: &mut 10 };

    *r.x = 1;
    *r.y = 11;

    println!("{}, {}", r.x, r.y);
  }


  let x: Message = Message::ChangeColor(3, 5, 6);


  let x = 5;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
  }

  process_message(Message::ChangeColor(10, 145, 200));
}

fn process_message(msg: Message)
{
  match msg
  {
    Message::Quit => quit(),
    Message::ChangeColor(r, g, b) => change_color(r, g, b),
    Message::Write(s) => println!("{}", s),
    _ => println!("messed up"),
  };
}

fn quit()
{
  println!("quit");
}

fn change_color(r: u32, g: u32, b: u32)
{
  println!("{}, {}, {}", r, g, b);
}