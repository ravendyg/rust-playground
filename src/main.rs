fn main()
{
  let mut x: i32 = 10;
  let y;

  x = 5;
  y = "sdff";
  // (x, y) = (5, 6);
  println!("{}, {:p}", x, y);
  print_num(x);

  print_sum(x, add_one(10));
  // diverges();

  let f: fn(i32) -> i32 = add_one;

  println!("{}", f(3));

  let a = [1,2,3,4,5];
  let ct = &a[..];
  let md = &a[1..3];

  println!("{}, {}", a[1], md[1]);


  let x: (i32, &str) = (1, "hello");

  println!("{}", x.1);

  assert_eq!(6, add_one(5));

  let y =
    if true
    {
      12
    }
    else
    {
      1
    };

  println!("{}", y);

  for (i, val) in (1..4).enumerate()
  {
    println!("{}, {}", i, val);
  }

  'outer: for x in 0..4 {
    'inner: for y in 0..4 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
  }

  let mut v = vec![1,2,3,4,5];

  // let j: u32 = 2;
  let j: usize = 2;
  println!("{}", v[j]);


  let mut k: i32 = 1;

  match v.get(10)
  {
    Some(x) => println!("{}", x),
    None    => println!("out of bound")
  };

  for mut i in &v
  { // wtf?
    i = &k;
    println!("{}: {}", i, k);
  }

  for i in &v
  {
    println!("{}", i);
  }
}

fn print_num(x: i32)
{
  println!("{}", x);
}

fn print_sum(x: i32, y: i32)
{
  println!("sum: {}", x + y);
}

fn add_one(x: i32) -> i32
{
  x + 1
}

fn diverges() -> !
{
  panic!("never returns");
}