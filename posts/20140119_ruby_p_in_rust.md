---
title: Ruby p in Rust aka puts driven development
date: 19.01.2014
categories: ["rust"]
tags: ["rust:macros", "rust:ruby"]
---

Given the lack of a useful debuggers in Ruby, we often refer to `p obj` to print an object to the console in order to inspect its value.

```ruby
Foo = Struct.new(:bar)
foo = Foo.new(7)
p foo # => #<struct Foo bar=7>
```

Actually you can do pretty much the same in Rust, too:

```rust
struct Foo {
  bar: uint
}

fn main() {
  let foo = Foo { bar: 7u };
  p!(foo); // => Foo{bar: 7u}
}
```

Actually Rust does not come from the factory with a `p!` - a syntax extension or a macro, but we can write it ourselves:

```rust
#[feature(macro_rules)] ;

macro_rules! p(
    ($ident:ident) => (
        println!("{:?}", $ident);
    );
)
```

For more on macros see the [Macro tutorial](http://static.rust-lang.org/doc/master/guide-macros.html)
