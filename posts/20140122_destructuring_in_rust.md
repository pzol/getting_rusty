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

Also it works in a function

```rust
fn my_function((a, b) : (uint, uint)) -> uint {
  a + b
}

fn main() {
  let pair = (1, 2);
  my_function(pair); // => 3
}
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
#[test]
  let v = ~[1, 2, 3, 4, 5];
  match v {
    []             => println!("empty"),
    [head]         => println!("{}", head),   // => 1
    [a, b, ..tail] => println!("{:?}", tail)  // => [3, 4, 5]
  }
}
```

For comments head over to [Reddit](http://www.reddit.com/r/rust/comments/1vu6v5/examples_of_destructuring_in_rust/)
