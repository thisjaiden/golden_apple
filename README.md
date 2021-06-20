# Overview
`golden_apple` is a library for decoding, encoding, and using common types found in Minecraft: Java Edition.

# Goals
- Provide a generalized format for sharing and using Minecraft's data types
- Simplify the decoding and encoding of network data

# Usage
Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top level crate.  
Types that can be fully represented in Rust have encoders/decoders under `golden_apple::generalized`, in case it isn't striaghtforward to do so.