# Danube Programming Language

Danube is a programming language strongly inspired by [Rust](https://github.com/rust-lang/rust/), but it is an interpreter language without a lifetime and ownership model.

## Hello, World!

```danube
fn main() {
  let message = "Hello, World!";
  println(message);
}
```

## Syntax

### Variables

```danube
let x = 10;
let mut name = "Danube";
```

### Conditions

```danube
if x > 5 {
  println!("Hello, Danube!");
} else {
  println!("Hello, World!");
}
```

### Loops

```danube
for i in 1..5 {
  println!(i);
}
```

### Function Declarations and Calling

```danube
fn add(a: u8, b: u8) -> u8 {
  a + b
}

add(1, 2); // 3
```

### Struct Declarations and Instantiation

```danube
struct User {
  name: String,
}

impl User {
  fn name(self) -> String {
    self.name
  }
}

let user = User {
  name: "Danuel",
}

println!(user.name());
```

### Module System

```danube
// math.dnb
pub fn add(a: u8, b: u8) -> u8 {
  a + b
}

// main.dnb
use math::add;

add(1, 2); // 3
```

### Context Receivers

```danube
fn say[f: Debug](user: User) {
  f.debug("Hello, ${user.name}!");
}

impl User {
  pub fn say[f: Debug](self) {
    f.debug("Hello, ${self.name}");
  }
}
```

## Special Thanks

This language was named Danube by [@thehighestend](https://github.com/thehighestend)!

## License

This project is licensed under the MIT license. Please see the LICENSE file for more details.
