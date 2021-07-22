# Overview
`golden_apple` is a library for decoding, encoding, and using common types found in Minecraft: Java Edition.

# Goals
- Provide a generalized format for sharing and using Minecraft's data types
- Simplify the decoding and encoding of network data

# Usage
Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top level crate. NBT is dealt with in the `nbt` module. 
Types that can be fully represented in Rust have encoders/decoders under `golden_apple::generalized`, in case it isn't striaghtforward to do so.

# Status
This crate is unfinished and some features aren't yet present. Here's an overview of what is and isn't done:

- [X] Standard Java Types  
- [X] String  
- [ ] Chat
- [ ] Identifier  
  Pretty much just a String, not sure if this should be included
- [X] VarInt
- [X] VarLong
- [ ] Metadata
- [ ] Slot
- [X] NBT  
- [X] Position
- [X] Angle
- [ ] UUID
- [ ] Enums  
  This refers to a variety of actual things
  