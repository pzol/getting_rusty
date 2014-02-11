
```rust
struct Foo<'a> {
  name: &'a str
}

let s = ~"foo";
let mut foo = Foo { name: &s };
```
error: mismatched types: expected `&str` but found `&~str` (expected &str but found &-ptr)
let mut foo = Foo { name: &s };

