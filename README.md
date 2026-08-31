# Nox

> A modern, compiled programming language focused on simplicity, performance and control.

Nox is a compiled programming language currently in the design phase.

The goal is to create a language that combines a simple and readable syntax with native performance, low-level control, and a safe memory model.

## Why Nox?

Nox aims to find a balance between several worlds:

- the simplicity and readability of high-level languages;
- the performance of compiled languages;
- the control offered by low-level languages;
- the safety provided by a strict type system;
- memory management without a global Garbage Collector;
- native interoperability with C.

Nox is not designed to replace a specific language and does not aim to copy Python, Lua, C, or Rust.

## Example

```nx
fn main() {
    stck name = "Nox"

    printf("Hello {name}")
}
```

## Variables

```nx
stck x = 10
const MAX = 100

x += 5
```

`stck` declares a mutable variable.

`const` declares an immutable value.

Types are strict: a variable cannot change its type after declaration.

## Conditions

```nx
if x > 10 {
    printf("large")
} elseif x == 10 {
    printf("equal")
} else {
    printf("small")
}
```

Conditions must strictly be of type `bool`.

There is no implicit truthy/falsy conversion.

## Loops

```nx
for i in 0..10 {
    printf(i)
}
```

```nx
while condition {
    ...
}
```

```nx
loop {
    ...
}
```

`break` and `continue` are available.

## Functions

```nx
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Parameters must be explicitly typed.

Default parameters and variadic functions are not part of the core language.

## References

Nox distinguishes between read-only and mutable references:

- `ref<T>`
- `mut ref<T>`

Example:

```nx
fn increment(value: mut ref<i32>) {
    value += 1
}
```

The reference system is designed around borrowing rules and memory management without a global Garbage Collector.

## Type System

Nox currently provides the following primitive types:

```text
i8   i16   i32   i64
u8   u16   u32   u64

f32  f64

bool
char
str
void
```

The language also provides:

- fixed-size arrays known at compile time;
- structs;
- enums;
- data-carrying enums;
- optionals `T?`;
- `Result<T, E>`;
- generics;
- traits.

## Optionals

Nox uses the `T?` syntax.

```nx
stck value: i32? = nil

if value != nil {
    ...
}
```

`nil` is only compatible with an optional type.

The compiler can automatically refine the type after a `!= nil` check.

## Enums

Simple enums are supported:

```nx
enum Color {
    Red
    Green
    Blue
}
```

Data-carrying enums are also supported:

```nx
enum Result<T, E> {
    Ok(T)
    Err(E)
}
```

## Match

`match` is exhaustive.

A missing variant causes a compilation error.

## Generics

Generics use the `<T>` syntax:

```nx
fn identity<T>(value: T) -> T {
    value
}
```

Generic constraints and traits are supported.

```nx
trait Comparable {
    ...
}
```

## Error Handling

Nox provides two separate systems.

### Exceptions

```text
try
catch
throw
```

### Result

```text
Result<T, E>
```

The `?` operator can propagate a `Result`.

`try/catch/throw` and `Result` are intentionally separate systems.

## Memory

Nox does not use a global Garbage Collector.

The current direction is based on:

```text
ownership
+
borrowing
+
references
```

The complete ownership and move model will be developed progressively.

## Unsafe

Dangerous low-level operations explicitly use:

```nx
unsafe {
    ...
}
```

Pointers requiring `unsafe` will be handled as part of this system.

## Pointers

Low-level pointers are planned.

Their detailed design will be defined later.

## Imports

Standard import:

```nx
import math
```

Alias:

```nx
import math as m
```

Selective import:

```nx
import math.sqrt
```

Each import is written on a separate line.

## Packages

Nox will provide a package system.

The package format and package manager will be defined later.

## Standard Library

The standard library is planned to include:

```text
math
fs
net
process
```

Planned functions include:

```text
printf
input
assert
```

## Compile-time

Nox supports compile-time-computable constants.

Example:

```nx
const SIZE = 10 + 20
```

A `comptime` mechanism is also planned.

## C FFI

Nox will provide interoperability with C.

Syntax:

```nx
extern fn foo(...)
```

The FFI will allow Nox programs to use C libraries.

ABI details, data layout, calling conventions, and pointer interoperability will be defined later.

## Operators

### Arithmetic

```text
+
-
*
/
//
//
%
```

`/` represents floating-point division.

`//` represents integer division.

### Assignment

```text
=
+=
-=
*=
/=
%=
```

Assignments are statements, not expressions.

Nested assignments such as `a = b = c` are forbidden.

### Logical

```text
and
or
not
```

### Comparisons

All comparison operators have the same precedence level.

They are left-associative.

The behavior of chained comparisons such as `x < y < z` remains to be defined.

### Bitwise

```text
&
|
^
~
shl
shr
```

## Syntax

Blocks must use `{}`.

Indentation is not significant.

Semicolons are allowed but optional.

Example:

```nx
stck x = 10;
stck y = 20
```

Unnecessary parentheses are allowed.

Trailing commas are allowed wherever the syntax accepts them.

## Visibility

Nox uses three visibility states:

- `pub` → public
- `prv` → explicitly private
- no keyword → visibility inherited from the surrounding context

## Methods

Methods use `impl`.

Example:

```nx
impl Player {
    fn damage(self: mut ref<Player>, amount: i32) {
        self.hp -= amount
    }
}
```

Explicit constructors are supported:

```nx
Player.new(...)
```

## Global Variables

Global variables are allowed.

Global constants are also allowed.

Shadowing is allowed in child scopes.

Redeclaration within the same scope is forbidden.

## `defer`

Nox supports `defer`:

```nx
defer {
    close(file)
}
```

The exact semantics will be defined alongside the runtime.

## Features Not Currently Planned

The following features are not part of the current foundations of Nox:

- macros;
- reflection;
- ternary operator;
- `++` / `--`;
- lambdas;
- dynamic collections;
- detailed ownership rules;
- detailed pointer rules;
- multiline strings;
- unions;
- tuples.

Some of these may be added later.

## Compiler Architecture

The current compiler direction is:

```text
.nx
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Semantic Analysis
 ↓
Type Checker
 ↓
IR
 ↓
Optimizations
 ↓
Native Code
 ↓
Linking
 ↓
Executable
```

The final backend technology, such as LLVM or another solution, has not yet been decided.

## Roadmap

### Phase 1 — Design

- [x] Primitive types
- [x] Variables
- [x] Constants
- [x] Operators
- [x] Conditions
- [x] Loops
- [x] Functions
- [x] Fixed-size arrays
- [x] Structs
- [x] Enums
- [x] References
- [x] Optionals
- [x] Result
- [x] Generics
- [x] Traits
- [x] Imports
- [x] C FFI
- [ ] Finalize the specification

### Phase 2 — First Compiler

- [ ] Lexer
- [ ] Parser
- [ ] AST
- [ ] Semantic analysis
- [ ] Type checker
- [ ] IR
- [ ] Code generation
- [ ] First Nox executable

### Phase 3 — Real-world Usage

- [ ] Write small Nox programs
- [ ] Identify design problems
- [ ] Stabilize the syntax
- [ ] Stabilize the type system
- [ ] Test performance

### Phase 4 — Advanced Systems

- [ ] Complete ownership
- [ ] Complete borrowing
- [ ] Move semantics
- [ ] Pointers
- [ ] Advanced generics
- [ ] Advanced traits
- [ ] Complete C FFI
- [ ] Runtime
- [ ] Packages
- [ ] Optimizations

### Phase 5 — Tooling

- [ ] Compiler CLI
- [ ] Formatter
- [ ] Package manager
- [ ] Advanced diagnostics
- [ ] Debugger
- [ ] Cross-compilation

## Philosophy

Nox is built around five principles:

```text
simplicity
+
performance
+
control
+
safety
+
consistency
```

Every feature should have a reason to exist.

The language should avoid accumulating features simply because they exist in other languages.

Nox should remain simple enough to understand and implement while still allowing low-level and systems programming.

## Development

The project follows this process:

```text
understand
→
decide
→
document
→
implement
→
test
```

Important language decisions should be documented in `nox_specs.md` before implementation.

The codebase should remain modular, tested, and documented.

Large monolithic files should be avoided.

## Documentation

The complete language specification is located in:

`nox_specs.md`

The README presents the project and its main concepts.

`nox_specs.md` is the source of truth for the precise rules of the language.

## Status

Nox is currently in the **foundation design phase**.

The syntax and some features may still evolve before the language is stabilized.

## License

To be defined.