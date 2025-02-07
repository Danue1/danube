# Danube Programming Language

Danube: For A New Era Language Standard.

It supports the basic features that are refined with reference to the trial and failure of existing languages.

## Hello, World!

```danube
fn main() {
  println("Hello, World!");
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
  println("Hello, Danube!");
} else {
  println("Hello, World!");
}
```

### Loops

```danube
for i in 1..5 {
  println("For! {i}");
}

while condition {
  println("While!");
}

loop {
  println("Loops!");
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
  fn name(self) -> str {
    self.name
  }
}

let user = User {
  name: "Danuel",
}

println(user.name());
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
math::add(1, 2); // 3
```

### Context Parameters

```danube
fn say(|f: mut Debug|, user: User) {
  f.debug("Hello, {user.name}!");
}

impl User {
  pub fn say(self, |f: mut Debug|) {
    f.debug("Hello, {self.name}");
  }
}

impl<T1> T1 {
  fn with<U>(self, f: (|T1|) -> U) -> U {
    f(|self|)
  }
}

// Formatter has a Debug trait.
Formatter::new().with(fn (|formatter|) {
  let user = User::new("Danuel");

  // 1 will be same as 2
  say(user); // 1
  say(|formatter|, user); // 2
})
```

### Traits

```danube
trait Identity {
  fn identity(self) -> Id;
}

struct User {
  id: Id,
}

impl Identity for User {
  fn identity(self) -> Id {
    self.id
  }
}
```

### Effect System

```danube
trait ^DivByZero {
  fn on_div_by_zero() -> u8;
}

fn div(a: u8, b: u8): ^DivByZero -> u8 {
  if b == 0 {
    ^DivByZero::on_div_by_zero()
  } else {
    a / b
  }
}
```

## Special Thanks

This language was named Danube by [@thehighestend](https://github.com/thehighestend)!

## License

This project is licensed under the MIT license. Please see the LICENSE file for more details.
