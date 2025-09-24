# bitmap

Procedural macro for defining bitmap structs packed as tightly as possible.  

---

## Features

- Define fields with `u1` to `u7` widths
- Up to 64 bits total per struct
- Auto-generates getters and setters
- Single `u64` backing storage

---

## Example

```rust
use bitmap::bitmap;

bitmap!(
    struct Player {
        imposter: u1,
        finished_tasks: u3,
        kills: u3,
    }
);

let mut p = Player(0);
p.set_imposter(1);
p.set_finished_tasks(5);
p.set_kills(3);

assert_eq!(p.finished_tasks(), 5);
assert_eq!(std::mem::size_of::<Player>(), 1);
```

# Limitations

* Total struct size must be ≤ 64 bits
* Only `u1` to `u7` types are currently supported (more planned)
