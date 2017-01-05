

struct Foo<'a>
{
  x: &'a mut i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> & i32
    {
      self.x
    }
}

fn main()
{
  println!("working");

  let mut v = vec![1, 2, 3];

  v.push(10);

  let r = refff(&v);
  let m = borrow(&mut v);

  for i in &mut v
  {
    *i += 1;
  }

  for i in &v
  {
    println!("{}", i);
  }

  let mut x = 1;

  {
    let y = &mut x;
    *y = 10;

    // println!("x: {}, y: {}", x, y);  // won't work with a borrowed binding
    println!("y: {}", y);
  }

  // *y = 10;

  println!("x: {}", x);

  let i = 1;
  let r;
  {

    r = &i;
  }

  println!("{}", r);

  let line = "lang:en=Hello World!";
  let lang = "en";

  let v;
  {
      let p = format!("lang:{}=", lang);  // -+ p goes into scope
      v = skip_prefix(line, p.as_str());  //  |
  }                                       // -+ p goes out of scope
  println!("{}", v);

  let y = &mut 18;
  let f = Foo { x: y };

  *f.x += 1;
  // *y += 1;

  println!("{}", f.x());

}

fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    line
}

fn refff(v: &Vec<i32>) -> i32
{
  42
}

fn borrow(v: &mut Vec<i32>) -> i32
{
  v[1] = 42;
  42
}


fn take(v: Vec<i32>)
{

}