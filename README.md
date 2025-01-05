## Example usage
```rust
let mut x = 6;
dowhile!(x < 3, {
   println!("x = {x}");
   x += 1;
});

// x = 6
```

```rust
let mut x = 6;
dowhile!({
    println!("x = {x}");
    x += 1;
} x < 3);

// x = 6
```
  
```rust
let mut x = 10;
dowhile!(x < 6, 'main_loop: {
    let mut y = x;
    println!("x = {x}");
    
    dowhile!({
        if y == 4 {
            break 'main_loop;
        }
        println!("y = {y}");
        y += 1;
    } y < x);
    
    x += 1;
});

// x = 10
// y = 0
// y = 1
// y = 2
// y = 3
```
