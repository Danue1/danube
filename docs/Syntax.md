# Syntax

## Comments

```fanube
// Comment
/* Comment */
```

## Variable Declarations

```danube
let a;
let b = 1;
let c: bool;
let d: bool = true;

let mut e;
let mut f = 1;
let mut g: bool;
let mut h: bool = true;
```

## Variable Types

```dabube
b = true; // bool
a = false; // bool
c = 1; // int
d = 1.1; // float
e = "abc"; // str
f = [1, 2, 3]; // array
f = (1, 2, 3); // tuple
g = { name: "Danube" }; // record
```

## Function Declarations

```danube
fn a() {
  println("Hello, World!");
}

fn a(b: int) {
  println("Hello, World!");
}

fn a() -> int {
  println("Hello, World!");
  return 1;
}
```

## Function Calls

```danube
a();
a(1);
a(1, 2, 3);
```

## Struct

```danube
// unnamed struct
struct Character(String);

// named struct
struct Character {
  name: String,
}
```

### Struct Instantiation

```danube
// unnamed struct
let character = Character("Hello, World!");
let character: Character = ("Hello, World!");
let character: Character = Character("Hello, World!");

// named struct
let character = Character { name: "Hello, World!" };
let character: Character = { name: "Hello, World!" };
let character: Character = Character { name: "Hello, World!" };
```

## Enum

```danube
enum PrimitiveType {
  S,
  I,
  B,
}
enum PrimitiveType {
  S(String),
  I(int),
  B(bool),
}
```

### Enum Instantiation

```danube
// enum PrimitiveType = S | I | B;
let primitive_type = PrimitiveType::S;
let primitive_type: PrimitiveType = PrimitiveType::S;
let primitive_type: PrimitiveType = S; // syntatic sugar

// enum PrimitiveType = S(String) | I(int) | B(bool);
let primitive_type = PrimitiveType::S("Hello, World!");
let primitive_type: PrimitiveType = PrimitiveType::S("Hello, World!");
let primitive_type: PrimitiveType = S("Hello, World!"); // syntatic sugar
```

## Control Flows

### Conditional Statements

```danube
// if
if true {
  println("Hello, World!");
}

// if -> else
if false {
  println("Unreachable!");
} else {
  println("Hello, World!");
}

// if -> else if
if false {
  println("Unreachable!");
} else if true {
  println("Hello, World!");
}

// if -> else if -> else
if false {
  println("Unreachable!");
} else if false {
  println("Unreachable!");
} else {
  println("Hello, World!");
}
```

### Loop Statements

```danube
// loop statement
loop {
  println("Hello, World!");
}

// while statement
while true {
  println("Hello, World!");
}

// for statement
for a in [1, 2, 3] {
  println(a);
}

// loop/while/for statement -> continue
loop {
  continue;
}

while true {
  continue;
}

for a in [1, 2, 3] {
  continue;
}

// loop/while/for statement -> break
loop {
  break;
}

while true {
  break;
}

for a in [1, 2, 3] {
  break;
}

// loop/while/for statement -> return
loop {
  return;
}

while true {
  return;
}

for a in [1, 2, 3] {
  return;
}
```

### Pattern Match Statements

```danube
// enum PrimitiveType = S | I | B;
match primitive_type {
  Primitive::S => { },
  Primitive::I => { },
  Primitive::B => { },
}

// enum PrimitiveType = S(String) | I(int) | B(bool);
match primitive_type {
  Primitive::S(s) => { }, // s == string
  Primitive::I(i) => { }, // i == int
  Primitive::B(b) => { }, // b == bool
}
```

## Module

```danube
use a::b; // import a::b
use a::{b, c}; // import a::b, a::c
use a::{b, c::d}; // import a::b, a::c::d
use a::b as c; // b alias to c

use self::a; // import ./a
use super::a; // import ../a
use mod::a; // import <root_module>::a
```
