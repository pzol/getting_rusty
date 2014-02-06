---
title: 'ParamsBuilder'
date: '2014-02-06'
description: 'Default arguments, named params, random order'
categories: ["rust"]
tags: []
---

Here is something modeled losely after [std::task::Task](http://static.rust-lang.org/doc/master/std/task/struct.TaskBuilder.html). The goal is to provide default parameters, named parameters and a random order of providing paramaters before doing something with it. 

```rust

#[deriving(Eq)]
struct ParamsBuilder {
    foo: uint,
    bar: ~str,
    baz: uint,
    run: bool
}

impl ParamsBuilder {
  fn default() -> ParamsBuilder{
    ParamsBuilder { foo: 0, bar: ~"", baz: 0, run: false }
  }

  fn foo(mut self, foo: uint) -> ParamsBuilder {
    self.foo = foo;
    self
  }

  fn bar(mut self, bar: ~str) -> ParamsBuilder {
    self.bar = bar;
    self
  }

  fn baz(mut self, baz: uint) -> ParamsBuilder {
    self.baz = baz;
    self
  }

  fn run(mut self) -> ParamsBuilder {
    self.run = true;
    println!("{:?}", self);
    self
  }
}


#[test]
fn test_chain(){
  let params = ParamsBuilder::default().foo(1).bar(~"bar").baz(2).run();

  assert_eq!(ParamsBuilder { foo: 1, bar: ~"bar", baz: 2, run: true }, params);
}
```

Thanks to Yurume for the help on this.


For comments head over to [Reddit](http://www.reddit.com/r/rust/comments/1x67wq/rust_by_example_default_arguments_named_params/)
