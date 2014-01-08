---
title: ToStr
date: 08.01.2014
description: How to get (almost) any type to support `to_str`
tags: ["deriving", "ToStr"]
categories: ["rust"]
---
In order to have a default implementaton of `to_str()` you need to add `#[deriving(ToStr, Rand)]`


```rust
#[deriving(ToStr, Rand)]
enum Weapons {
  Sword,
  Club,
  Gaze
}

println(Club.to_str);

// => Club
```

The alternative, workin in most cases, would be using `format!`

```rust
enum Monsters {
  Goblin,
  Orc
}

fn main() {
  println(format!("{:?}", Orc));
}

// => Orc
```
