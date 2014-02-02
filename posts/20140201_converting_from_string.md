---
title: Converting from ~str
date: '2014-02-01'
description: 
tags: ['FromStr']
categories: ["rust"]
---

## Converting from String

When converting from string to numbers, you will have to provide the type manually

```rust
let f = from_str("1.2");
// -> error: cannot determine a type for this bounded type parameter: unconstrained type

let f: f32 = from_str("1.2").unwrap(); // -> 1.2f32

// or like this
let i = from_str::<uint>("5").unwrap();
assert_eq!(i, 5);
```

The trait [FromStr](http://static.rust-lang.org/doc/master/std/from_str/trait.FromStr.html) is defined as

```rust
pub trait FromStr {
    fn from_str(s: &str) -> Option<Self>;
}
```

so it returns a Some<T> if the conversion succeeds or a None if it doesn't

```rust
// Some<T>
let oi: Option<uint> = from_str("1");
assert_eq!(oi, Some(1u));

// None, if the provided string cannot be converted
let oi: Option<uint> = from_str("x");
assert_eq!(oi, None);
```

You can `unwrap()` the value

```rust
let i: uint = from_str("1").unwrap();
assert_eq!(i, 1);

// runtime error 'called `Option::unwrap()` on a `None` value'
let i: uint = from_str("x").unwrap();
assert_eq!(i, 0);
```

As you can see, you can unwrap the value straight away, but that might give you runtime errors, hence it'd be better to check if the conversion succeeded and provide a meaningful message. Alternatively, you can provide a default value using `unwrap_or()`.

```rust
let i: uint = match from_str("1") {
  Some(value) => value,
  None        => fail!("oops, expected a number")
}

let i: uint = from_str("4").unwrap_or(0u);
assert_eq!(i, 4);
```

For comments head over to [Reddit](http://redd.it/1wr1ct)
