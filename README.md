schemers
========

`schemers` is a Scheme implementation in Rust.

Requirements
------------

- Rust: It should be installed via [`rustup`](https://rustup.rs). If you already
  have Rust, make sure you've ran `rustup update` to make sure you have the
  latest version.

Compilation
-----------

As with any Rust project, the way to compile this is to just run:

```shell
$ cargo build --release
[ ... output ... ]
```

Running
-------

If you have a file named `example.scm`, you can just run:

```shell
$ cargo run --release example.scm
[ ... your program's output ... ]
```

License
-------

MIT
