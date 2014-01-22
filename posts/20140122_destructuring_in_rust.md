---
title: Destructuring in Rust
date: 22.01.2014
categories: ["rust"]
tags: ["rust:enum", "rust:destructuring"]
---
Here's another piece I came along, while learning [Rust](http://rust-lang.org).
Pattern matching is one of the features I like most about modern / functional style languages, also one I sincerely enjoy in [Rust](http://rust-lang.org).

It works in a lot of different scenarios, the most basic in a `let`

```rust
let tuple = (1, 2);
let (a, b) = tuple; // => a =  1; b = 2
```

It works in a function's arguments:

```rust
fn my_function((a, b) : (uint, uint)) -> uint {
  a + b
}

fn main() {
  let pair = (1, 2);
  my_function(pair); // => 3
}
```

You can destructure structs and rename the variables:

```rust
let  p = Point { x: 1, y: 2 };
let  Point { x: new_x, y: new_y } = p; // => new_x == 1, new_y == 2
assert_eq!(new_x, 1);
assert_eq!(new_y, 2);
```

The order is not important:

```rust
let Point { y, x } = p;        // => y  == 2, x  == 1
let Point { y: y2, x: x2} = p; // => y2 == 2, x2 == 1
```

and you can also ignore some variables:

```rust
  let Point { y: y3, .. } = p; // => y3 == 2
```


It also can be used to destructure struct variants:

```rust
#[feature(struct_variant)];

enum Fagazi {
  Fob { a: int },
  Foo { b: int, c: int }
}

#[test]
fn test_enum() {
  let foo = Foo { b: 1, c: 2 };

  match foo {
    Foo { b, c } if b > 2 => assert!((b,c) == (1, 2)),
    Foo { b, c } => assert!((b,c) == (1, 2)),
    _            => fail!("This will never happen, but the compiler doesn't know")
  };
}
```

You __cannot__ just destructure an enum with multiple variants using `let`:

```rust
let Foo { b, c } = foo; // ! Compiler error !
```

You need to use a match instead of a simple let, because let can never fail (refutable pattern in local binding)
using the second condition in match, the compiler knows, all possible paths have been exhausted.


One more cool feature of `match` are guard clauses:

```rust
fn test_enum() {
  let foo = Foo { b: 3, c: 2 };

  match foo {
    Foo { b, c } if b <= 2 => assert!(b <= 2 && c == 2),
    Foo { b, c }           => assert!((b, c) == (3, 2)),
    _                      => unreachable!()
  };
}
```

See the `if b <= 2` in the first line? This is called a guard, it will match only if the pattern matches and the guard clause is true.

Take also notice of the `unreachable!()` expression. As mentioned before all `match` clauses need to be exhaustive. `unreachable!()` expands to `fail!("internal error: entered unreachable code")`.


You can destructure vectors, too:

```rust
let v = ~[1, 2, 3, 4, 5];
match v {
    []                       => println!("empty"),
    [elem]                   => println!("{}", elem),   // => 1
    [first, second, ..rest]  => println!("{:?}", rest)  // => [3, 4, 5]
  }
```

If you only care about the first or last values you can do this:

```rust
let v = ~[1, 2, 3];
match v {
  [first, ..] => assert_eq!(first, 1),
  [.., last]  => assert_eq!(last, 3),
  _           => unreachable!()
}
```

You can also use destructuring in `for` loops:

```rust
struct Pair { x: int, y: int }

let pairs = ~[Pair {x: 10, y: 20}, Pair {x: 30, y: 0}]; 
  
for &Pair {x, y} in pairs.iter() {
  assert_eq!(x + y, 30);
}
```

For comments head over to [Reddit](http://www.reddit.com/r/rust/comments/1vu6v5/examples_of_destructuring_in_rust/)
