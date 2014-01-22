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
