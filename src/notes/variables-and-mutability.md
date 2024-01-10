# Variables and Mutability
| By default, variables are immutable.

## let & mut

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
## Constants

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Constant vs Variable

1. First, you aren’t allowed to use mut with constants. 
2. The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

## Shadowing

In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6
}
```


```rust
let spaces = "   ";
let spaces = spaces.len();

```

We’re not allowed to mutate a variable’s type:
```rust
let mut spaces = "   ";
spaces = spaces.len();

```