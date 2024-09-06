# Changelog

## Unreleased

- The `netty` module is a new module with types and tools for communicating with standard minecraft servers and clients. As of now, it only supports the handshake/status/login phases of networking.
- `enums::ProtocolState` is now `netty::ProtocolState`
- `netty::ProtocolState` has been updated to support the "configuration" protocol state
- `enums::NextState` is now `netty::handshake::NextState`
- Fixed some parsing bugs in the `generalized` submodule relating to improper use of `[V; N]` syntax
- The following NBT parsing functions are now public interfaces:
  - `nbt::read_tag_by_type`
  - `read_tag_with_type`
  - `read_named_tag`
- Fixed the `VarLong` type.
- `VarInt` and `VarLong` both now have a `read_size` proprety, which returns the amount of bytes needed to read the value from a datastream, if it was aquired that way.
- Fixed a parsing bug with `UUID`s relating to the improper use of `[V; N]` syntax
- `UUID` is now a Copy type
- `generalized::string_from_bytes`, `generalized::string_from_reader`, `generalized::string_to_bytes`, and `generalized_string_to_writer` have been updated to convert to and from "normal" UTF-8 strings and Java's "Modified UTF-8"
  - These functions have variants ending in `_no_cesu8` that do not do this conversion.
- Fixed serialization and writing NBT to properly convert strings into Java's "Modified UTF-8."
- Updated dependencies to the latest versions:
  - `serde` 1.0.207 -> 1.0.209
  - `serde_json` 1.0.124 -> 1.0.127
  - `reqwest` 0.12.5 -> 0.12.7

## 0.19.0

- NBT string tags are now converted to valid "normal" UTF-8 strings
- Fixed some NBT parsing bugs ([#1](https://www.github.com/thisjaiden/golden_apple/issues/1))
- Added some NBT unit tests
- Updated `serde` and `serde_json`

## 0.18.0

- Updated to Minecraft 1.21.1 (protocol 767)
- Removed the Enchantments and Paintings enums (these are now data driven, and non-exaustive)

## 0.17.2

- Fixed username query API
- Updated dependencies
