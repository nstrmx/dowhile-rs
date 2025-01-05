## Example usage
```rust
let mut x = 6;
dowhile!(x < 3, {
   println!("x = {x}");
   x += 1;
});
```

```rust
let mut x = 6;
dowhile!({
    println!("x = {x}");
    x += 1;
} x < 3);
```
  
```rust
let mut x = 0;
dowhile!(x < 6, 'main_loop: {
    let mut y = x;
    println!("x = {x}");
    
    dowhile!({
        if y == 4 {
            break 'main_loop;
        }
        println!("y = {y}");
        y += 1;
    } y < 6);
    
    x += 1;
});
```
