# Danube Programming Language

Danube is a programming language strongly inspired by [Rust](https://github.com/rust-lang/rust/), but it is an interpreter language without a lifetime and ownership model.

## Hello, World!

```danube
fn main() {
  let message = "Hello, World!";
  println(message);
}
```

The syntax can be found in [docs/Syntax.md](./docs/Syntax.md).

## Special Thanks

This language was named Danube by [@thehighestend](https://github.com/thehighestend)!

## License

This project is licensed under the MIT license. Please see the LICENSE file for more details.

```danube
fn main() {
  let message: Message = { raw: "Hello, World!" };
  println(message);

  let message = Message { raw: "Hello, World!" };
  println(message);

  let message: Message = Message { raw: "Hello, World!" };
  println(message);

  let message: Message = ("Hello, World!");
  println(message);

  let message = Message("Hello, World!");
  println(message);

  let message: Message = Message("Hello, World!");
  println(message);
}
```
