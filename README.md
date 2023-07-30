## SCPCB Redux Mapper

[![crates.io](https://img.shields.io/crates/v/scpcb_redux_mapper.svg)](https://crates.io/crates/scpcb_redux_mapper) [![docs.rs](https://docs.rs/scpcb_redux_mapper/badge.svg)](https://docs.rs/scpcb_redux_mapper)

### Usage

```rust
let seed = seed_from_string("usage");
let mut map = Map::new(16, 16, seed);
while map.room_1_amount < 2 {
    map.generate(24);
}
```

#### Similar Projects
- [Godot C# SCPCB Mapgen](https://github.com/Yni-Viar/scp-mapgen-v3)
- [Map Generation C++](https://undertowgames.com/forum/viewtopic.php?f=11&t=5265#p107608)
