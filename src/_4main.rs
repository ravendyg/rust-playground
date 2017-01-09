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
<<<<<<< HEAD
=======

  let m = Message::Write("Hello, world".to_string());


  let v = vec!["Hello".to_string(), "World".to_string()];
  // let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();

  let q: Vec<i32> = v.into_iter().map(printStr).collect();

  let point3 = Point3d {x: 3, y: 34, z: 2};

  match point3
  {
    Point3d {x, y, ..} => println!("{}, {}", x, y),
  }

  let ch = 'X';

  match ch
  {
    'a' ... 'x' | 'A' ... 'X' => println!("matched"),
    _ => println!("didn't"),
  }
}

fn printStr(s: String) -> i32
{
  println!("{}", s);
  111
>>>>>>> 07c2273765eacbe14a1d34e1f823819b76b526dd
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
<<<<<<< HEAD
=======
}

fn move_cursor(x: u32, y: u32)
{
  println!("move cursor to ({}, {})", x, y);
>>>>>>> 07c2273765eacbe14a1d34e1f823819b76b526dd
}