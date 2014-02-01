---
title: Destructuring and Pattern Matching
date: 22.01.2014
description: 
categories: ["rust"]
tags: ["rust:enum", "rust:destructuring"]
---

Pattern matching is one of the features I like most about modern / functional style languages, also one I sincerely enjoy in [Rust](http://rust-lang.org).

It works in a lot of different scenarios, the most basic is in a local scope using `let`.

```rust
let tuple = (1, 2);
let (a, b) = tuple; // => a =  1; b = 2
```

## Structs
Should you have the need to capture a nested tuple or something, you can do that with the Haskell @ syntax:

```rust
struct Foo { x: (uint, uint), y: uint }

let foo = Foo { x: (1, 2), y: 3 };
let Foo { x: tuple @ (a, b), .. } = foo; // => a == 1; b == 2; tuple == (1, 2)
```

You can destructure structs and rename the variables:

```rust
<<<<<<< HEAD
struct Point { x: uint, y: uint }

let  p = Point { x: 1, y: 2 };
let  Point { x: new_x, y: new_y } = p; // => new_x == 1, new_y == 2
=======
let  p = Point { x: 1, y: 2 };
let  Point { x: new_x, y: new_y } = p; // => new_x == 1, new_y == 2
assert_eq!(new_x, 1);
assert_eq!(new_y, 2);
>>>>>>> e71adc5ca7af0e8c5ce50607f42602505182c44c
```

The order is not important:

```rust
let Point { y, x } = p;        // => y  == 2, x  == 1
let Point { y: y2, x: x2} = p; // => y2 == 2, x2 == 1
```

and you can also ignore some variables:

```rust
<<<<<<< HEAD
let Point { y: y3, .. } = p; // => y3 == 2
let Point { y } = p;         // -> error: pattern does not mention field `x`
```

you can match on ranges

```rust
let b = match 5 { 0..5 => true, _ => false}; // => true
```

## Struct Variants
=======
  let Point { y: y3, .. } = p; // => y3 == 2
  let Point { y } = p;         
```


>>>>>>> e71adc5ca7af0e8c5ce50607f42602505182c44c
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
let Foo { b, c } = foo; // -> error: refutable pattern in local binding
```

You need to use a match instead of a simple let, because let can never fail using the second condition in match, the compiler knows, all possible paths have been exhausted.

One more cool feature of `match` are guard clauses:

```rust
let foo = Foo { b: 3, c: 2 };

match foo {
  Foo { b, c } if b <= 2 => assert!(b <= 2 && c == 2),
  Foo { b, c }           => assert!((b, c) == (3, 2)),
  _                      => unreachable!()
};
```

See the `if b <= 2` in the first line? This is called a guard, it will match only if the pattern matches and the guard clause is true.

Take also notice of the `unreachable!()` expression. As mentioned before all `match` clauses need to be exhaustive. `unreachable!()` expands to `fail!("internal error: entered unreachable code")`.


`match` allows to match on concrete values:

```rust
let foo = Some(1);

match foo {
  Some(3) => println!("three"),
  Some(2) => println!("two"),
  Some(v) => println!("not two, {}", v),
  None    => unreachable!()
}
```

Remember, that a `match` must contain all possibilities, otherwise you'll get an error, saying that you haven't covered all patterns. In this case if you'd leave out the last `None`, the compiler would tell you:  `non-exhaustive patterns: None not covered`.

## Vectors
You can destructure vectors, too:

```rust
let v = ~[1, 2, 3, 4, 5];

match v {
    []                       => println!("empty"),
    [elem]                   => println!("{}", elem),   // => 1
    [first, second, ..rest]  => println!("{:?}", rest)  // => &[3, 4, 5]
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

or if you want the first, last, but also the middle:

```rust
let v = ~[1, 2, 3, 4, 5];

match v {
  [first, .. middle, last] => println!("{:?} {:?} {:?}", first, middle, last),
  _                        => unreachable!()
}
```

matching on a `~[str]` works just like matching any other vector

```rust
match ~[~"foo", ~"bar"] {
  [~"foo"] => 1, 
  _ => 2, 
}
```

## Function Arguments
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

## Loops
You can also use destructuring in `for` loops:

```rust
struct Pair { x: int, y: int }

let pairs = ~[Pair {x: 10, y: 20}, Pair {x: 30, y: 0}]; 
  
for &Pair {x, y} in pairs.iter() {
  assert_eq!(x + y, 30);
}
```

For comments head over to [Reddit](http://www.reddit.com/r/rust/comments/1vu6v5/examples_of_destructuring_in_rust/)
