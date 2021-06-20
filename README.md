# Overview
`golden_apple` is a library for decoding, encoding, and using common types found in Minecraft: Java Edition.

# Goals
- Provide a generalized format for sharing and using Minecraft's data types
- Simplify the decoding and encoding of network data

# Usage
Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top level crate.  
Types that can be fully represented in Rust have encoders/decoders under `golden_apple::generalized`, in case it isn't striaghtforward to do so.

# Status
This crate is in an early stage. Many features aren't yet present. Here's an overview of what is and isn't done:

- [X] Boolean
- [ ] String  
  Decoding done, encoding WIP
- [ ] Chat
- [ ] Identifier  
  Pretty much just a String, not sure if this should be included
- [X] VarInt
- [ ] VarLong
- [ ] Metadata
- [ ] Slot
- [ ] NBT
- [ ] Position
- [ ] Angle
- [ ] UUID
- [ ] Enums  
  This refers to a variety of actual things
  