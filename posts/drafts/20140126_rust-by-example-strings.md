---
title: 'Strings'
date: '2014-02-01'
description: 'Comparing, converting from, to, mutating'
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
