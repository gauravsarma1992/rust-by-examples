use std::fmt;
use std::fmt::Display;
use std::fmt::Result;


#[derive(Debug)]
struct UnPrintable(i32);

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

struct Displayable {
  age: u8
}

impl fmt::Display for Displayable {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result {
    write!(f, "Age is {}", self.age)
  }
} 

struct List(Vec<i32>);

impl Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result {
    let vec = &self.0;

    write!(f, "[")?;

    for (count, v) in vec.iter().enumerate() {
      if count != 0 {
        write!(f, ", ")?;
      }
      write!(f, "Count {} -> Value {}", count, v)?;
    }

    write!(f, "]")
  }
}


struct City {
  name: &'static str,
  lat: f32,
  lon: f32,
}

impl Display for City {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result {
    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

    write!(f, "{}: {:.3} degree {} {:.3} degree {}",
      self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
  }
}


fn main() {
  // Adding comments too
  
  let x = 10/5;
  println!("Hello world {}", x);
  println!("{x}, {y}, {z}", x=1, y=2, z=3);

  println!("{:#?}", UnPrintable(32));

  let name = "Gary";
  let age = 30;
  let person = Person { name, age };

  println!("{:#?}", person);

  let displayable = Displayable { age };
  println!("{}", displayable);

  let v = List(vec![1, 2, 3]);
  println!("{}", v);

  for city in [
      City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
      City { name: "Oslo", lat: 59.95, lon: 10.75 },
      City { name: "Vancouver", lat: 49.25, lon: -123.1 },
  ] {
    println!("{}", city);
  }

}
