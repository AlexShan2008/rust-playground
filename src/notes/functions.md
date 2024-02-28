# Functions

> Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Parameters

```rust
fn main() {
    another_function(5, 'h');
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}");
}
```

## Statements and Expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.
  This expression:

```rust
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

## Functions with Return Values

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```
