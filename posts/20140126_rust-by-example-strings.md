---
title: 'Strings'
date: 01.02.2014
description: 'Converting from, to, mutating'
tags: []
---

Here are some examples of string related functions I find interesting from stdlib.

## Comparing

The first thing that struck me when I first started with [Rust](http://rust-lang.org), was that a simple string comparison does not work, at least not the way you'd expect, because I was still oblivious of how pointers and friends work in [Rust](http://rust-lang.org).

```rust
let bilbo = ~"Bilbo Baggins"; // owned
if bilbo == "Bilbo Baggins" {};
// -> error: mismatched types: expected `~str` but found `&'static str` (str storage differs: expected `~` but found `&'static `)
```
That is because `bilbo` is an owned pointer and the literal `"Bilbo Baggins"` is a static borrowed pointer.

Those examples *will work*:

```rust
let bilbo = ~"Bilbo Baggins";

if "Bilbo Baggins" == bilbo {}; // [1] auto-borrow on .eq
if bilbo == "Bilbo Baggins".to_owned() {}; // convert &'static str to ~str
if bilbo.as_slice() == "Bilbo Baggins" {};
```

For some discussion about example _[1]_ see [here](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-07-30#wiki--and-autoderef).

Personally I would go for the last one, `.as_slice()`.

[TODO]: compare case insensitive

## Converting from String

When converting from string to numbers, you will have to provide the type manually

```rust
let f = from_str("1.2");
// -> error: cannot determine a type for this bounded type parameter: unconstrained type

let f: f32 = from_str("1.2").unwrap(); // -> 1.2f32
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
  Some<value> => value,
  None        => fail!("oops, expected a number")
}

let i: uint = from_str("4").unwrap_or(0u);
assert_eq!(i, 4);
```

## Converting to String

## String Functions

### empty?

```rust
assert_eq!((~"").is_empty(), true);
assert_eq!("".is_empty(), true);
assert_eq!("Bąk".is_empty(), false);
```

Defined in the [Container](http://static.rust-lang.org/doc/master/std/container/trait.Container.html#method.is_empty) trait.

### length
This is difficult. Tried to ask on #rust once and have regretted it since. You can use `char_len()` at your own peril.

```rust
let s = ~"Bär";
assert_eq!(s.char_len(), 3);
```

### Starts with, Ends with

Those two do a case-sensitive comparison.

```rust
let bilbo = ~"Bilbo Baggins";
assert_eq!(bilbo.starts_with("Bilbo"), true);
assert_eq!(bilbo.starts_with("bilbo"), false);
assert_eq!(bilbo.ends_with("Baggins"), true);
assert_eq!(bilbo.ends_with("baggins"), false);
```


## String Mutations

### Concat, Prepend, Append

You can concat two immutable string, this will of course return a new string.

```rust
let x = ~"foo";
let y = ~"bar";

let z = x + y; // -> ~"foobar"
let v = x.append(y); // -> ~"foobar"
```

If you really want to mutate a string, you have to make it mutable

```rust
let mut foo = ~"foo";
foo.push_char(' ');
foo.push_str("bar"); // -> ~"foo bar"
```

See also [OwnedStr](http://static.rust-lang.org/doc/master/std/str/trait.OwnedStr.html)

### Inserting

### Replace

```rust
let bilbo = ~"Bilbo Baggins";
let frodo = bilbo.replace("Bilbo", "Frodo"); // -> ~"Frodo Baggins"

// not found, no change
let samwise = bilbo.replace("Frodo", "Samwise"); // -> ~"Bilbo Baggins"
```

see [StrSlice#replace](http://static.rust-lang.org/doc/master/std/str/trait.StrSlice.html#tymethod.replace)

### Capitalize, Upcase, Downcase, Swapcase
`to_lower()` and `to_upper()` as they are called in [Rust](http://rust-lang.org), are currently only implemented in std only for [ascii](http://static.rust-lang.org/doc/master/std/ascii/struct.Ascii.html#method.to_lower) strings.

## TODO

* chomp
* delete

* include?
* index
* rindex

* insert
* lines
* ljust
* rjust
* lstrip
* rstrip
* strip
* next
* partition
* reverse
* sub
* gsub
* slice
* split
* squeeze
* tr
* truncate

* each_char

## Regex
* match 
* scan

## Notes & more
At time of writing 

```bash
$ rustc -v
rustc 0.10-pre (caabbb8 2014-01-25 06:31:31 -0800)
```

Some links to the stdlib documentation

* [std::str](http://static.rust-lang.org/doc/master/std/str/index.html)
* [std::from_str](http://static.rust-lang.org/doc/master/std/from_str/trait.FromStr.html)
* [std::to_str](http://static.rust-lang.org/doc/master/std/to_str/trait.ToStr.html)
* [std::str::StrSlice](http://static.rust-lang.org/doc/master/std/str/trait.StrSlice.html)
