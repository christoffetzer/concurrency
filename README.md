concurrency
===========

Crate concurrency is a set of functions to demonstrate the concurrency
primitives of Rust. In particular, it shows an approach on how to solve
the "sadistic homework" (see below) using the actor model.

The sadistic homework requires you to build a concurrent list data structure
in which one can enqueue elements on the "left" and the "right" and there
should be no interference in case the list has sufficient length. A 
solution to this problem was published in PODC1996 by Michael and Scott.  


### Installation

```sh
git clone https://github.com/christoffetzer/concurrency
cd concurrency
cargo build
cargo test
cargo doc
```

### Documentation

After you have built the documentation with

``sh
cargo doc
```

You can view the documentation with your browser by opening
file concurrency/target/doc/concurrency/index.html .





