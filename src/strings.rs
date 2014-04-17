#[test]
fn test_compare_fails(){
  let hobbit_name = ~"Bilbo Baggins";
  // if hobbit_name == "Bilbo Baggins" {}; // -> error

  if "Bilbo Baggins" == hobbit_name {};
  if hobbit_name == "Bilbo Baggins".to_owned() {};
  if hobbit_name.as_slice() == "Bilbo Baggins" {};
}

fn compare_ci(x: &str, y: &str) -> bool {
  let mut it = x.chars().zip(y.chars());
  it.all(|(x,y)|
    unsafe {
      x.to_ascii_nocheck().to_lower() == y.to_ascii_nocheck().to_lower()
    }
  )
}

#[test]
fn test_compare_case_insensitive(){
  let x = ~"Bilbo Bäggins";
  let y = ~"bILbo bäGgins";

  assert_eq!(compare_ci(x, y), true);
}

#[test]
fn test_ci(){
  let x = "Bilbo";
  let y = "bIlbo";
  // assert_eq!(x.to_lower(), y.to_lower());
}

#[test]
fn test_from_str(){
  let f: f32 = from_str("1.2").unwrap(); // 1.2f32
  assert_eq!(f, 1.2f32);

  let oi: Option<uint> = from_str("1");
  assert_eq!(oi, Some(1u));

  let oi: Option<uint> = from_str("x");
  assert_eq!(oi, None);

  let i: uint = from_str("2").unwrap();
  assert_eq!(i, 2);

  // let i: uint = from_str("x").unwrap();
  // assert_eq!(i, 0);

  let i: uint = match from_str("3") {
    Some(value) => value,
    None        => fail!("oops, expected a number")
  };
  assert_eq!(i, 3);

  let i: uint = from_str("4").unwrap_or(0u);
  assert_eq!(i, 4);


  let i = from_str::<uint>("5").unwrap();
  assert_eq!(i, 5);
}

#[test]
fn test_empty(){
  assert!((~"").is_empty());
  assert!((&"").is_empty());
  assert!(!"Bär".is_empty());
}

#[test]
fn test_length(){
  let s = ~"Bär";
  assert_eq!(s.char_len(), 3);
}


#[test]
fn test_concat(){
  let x = ~"abc";
  let y = ~"def";

  let z = x + y;
  assert_eq!(~"abcdef", z)

  let a = ~"foo";
  let b = ~"bar";
  let c = a.append(b);
  assert_eq!(~"foobar", c);
}

#[test]
fn test_mutate(){
  let mut foo = ~"foo";
  foo.push_str("bar");
  assert_eq!(~"foobar", foo);
}

#[test]
fn test_replace(){
  let bilbo = ~"Bilbo Baggins";
  let frodo = bilbo.replace("Bilbo", "Frodo");
  assert_eq!(~"Frodo Baggins", frodo);

  let samwise = bilbo.replace("Frodo", "Samwise");
  assert_eq!(samwise, bilbo);
}

#[test]
fn test_start_end_with(){
  let bilbo = ~"Bilbo Baggins";
  assert_eq!(bilbo.starts_with("Bilbo"), true);
  assert_eq!(bilbo.starts_with("bilbo"), false);
  assert_eq!(bilbo.ends_with("Baggins"), true);
  assert_eq!(bilbo.ends_with("baggins"), false);
}

#[test]
fn test_owned_borrowed() {
let mut owned: ~str    = ~"foobar";
let mut borrowed: &str = &"foobaz";

assert_eq!(owned, ~"foobar");
assert_eq!(borrowed, "foobaz");

owned    = borrowed.to_owned();
assert_eq!(owned, ~"foobaz");

borrowed = owned.as_slice();
assert_eq!(borrowed, &"foobaz");
}

// reference shit

struct Foo<'a> {
  name: &'a str
}

impl<'a> Foo<'a> {
  fn name(&'a mut self, s: &'a str) -> &'a mut Foo<'a> {
    self.name = s;
    self
  }
}

#[test]
fn test_in_struct(){
  let s = ~"foo";
  let mut foo = Foo { name: s.as_slice() };

  assert_eq!(foo.name, "foo");

  let foo = foo.name("bar");
  assert_eq!(foo.name, "bar");
}

#[test]
fn Converting_to_String(){
  let i = 1i32;
  assert_eq!(i.to_str(), ~"1");
  // let j = 2.to_str::<i8>();
  // assert_eq!(j, ~"2");
}

struct LowerChars<'a> {
  chars: std::str::Chars<'a>
}

fn lower<'a>(s: &'a str) -> LowerChars<'a> {
  LowerChars { chars: s.chars() }
}

impl<'a> Iterator<char> for LowerChars<'a> {
  fn next(&mut self) -> Option<char> {
    self.chars.next().map(|c| c.to_lowercase())
  }
}

#[test]
fn test_to_uppercase(){
  let sl = "foobär";
  let su = "FOOBÄR";

  let mut z = lower(sl).zip(lower(su));
  assert!(z.all(|(x, y)| x == y ));

  // let ss = "foobär".chars().map(|c| c.to_uppercase()).collect::<~str>();
  // assert_eq!(ss.as_slice(), su);
}
