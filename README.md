## Installation
```
cargo add dowhile_rs
```

## Example usage

```rust
let mut x = 6;
dowhile!({
    println!("x = {x}");
    x += 1;
} x < 3);

// x = 6
```

#### Pattern matching
```rust
let mut x = Some(6);
dowhile!({
    println!("x = {x:?}");
    x = Some(x.unwrap_or(0) + 1);
} let Some(..3) = x);

// x = Some(6)
```

```rust
let mut x = Some(6);
dowhile!({
    println!("x = {x:?}");
    x = Some(x.unwrap_or(0) + 1);
} let Some(val) = x => val < 3);

// x = Some(6)
```

#### Nested  
```rust
let mut x = 4;
dowhile!({
    println!("x = {x}");

    let mut y = 0;
    dowhile!({
        println!("y = {y}");
        y += 1;
    } y < x);
    
    x += 1;
} x < 4);

// x = 4
// y = 0
// y = 1
// y = 2
// y = 3
```

#### Labeled
```rust
let mut x = 10;
dowhile!('first: {
    println!("x = {x}");

    let mut y = 0;
    dowhile!('second: {
        if y == 4 {
            break 'first;
        }
        println!("y = {y}");
        y += 1;
    } y < x);
    
    x += 1;
} x < 6);

// x = 10
// y = 0
// y = 1
// y = 2
// y = 3
```
