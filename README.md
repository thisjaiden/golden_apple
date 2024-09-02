# Overview

`golden_apple` is a library for decoding, encoding, and using common types found in Minecraft: Java Edition.

## Goals

- Provide a generalized format for sharing and using Minecraft's data types
- Simplify the decoding and encoding of network data
- Abstract away enums usually passed as numbers

## Usage

### Parsing NBT

```rust, no_run
let mut nbt_file_reader = std::fs::File::open("test.nbt")
    .expect("Unable to open file!");

match golden_apple::nbt::from_reader(&mut nbt_file_reader) {
    Ok(named_tag) => {
        println!("NBT Data: {:#?}", named_tag);
    }
    Err(e) => {
        panic!("Unable to parse NBT! ({:?})", e);
    }
}
```

### Other

Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top level crate.
Types that can be fully represented in Rust have encoders/decoders under `golden_apple::generalized` for reading and writing from both byte arrays and Rust's Read/Write traits.
Communicating with existing servers and clients can be done using the packet format and tools found in the `netty` module.
<!-- TODO: put more real examples here, PLEASE. -->

## Status

This crate is unfinished and some features aren't yet present. Here's an overview of what is and isn't done:

- [X] Standard Java Types  
- [X] String  
- [X] Chat
- [X] Identifier
- [X] VarInt
- [X] VarLong
- [X] NBT  
- [X] Position
- [X] Angle
- [X] UUID
- [ ] Metadata
- [ ] Slot
- [ ] Netty (~2%)
- [ ] Other Enums (~50%)

## Version Support

[Changelog](changelog.md)
|  Crate version  | Minecraft version | Minecraft Protocol ID |
| --------------- | ----------------- | --------------------- |
| 0.18.0 - 0.19.0 | 1.21.0 - 1.21.1   | 767                   |
| 0.17.0 - 0.17.2 | 1.19.2            | 762                   |
