# Changelog

## Unreleased

- The `netty` module is a new module with types and tools for communicating with standard minecraft servers and clients. As of now, it only supports the "handshake" phase of networking.
- `enums::ProtocolState` is now `netty::ProtocolState`
- `netty::ProtocolState` has been updated to support the "configuration" protocol state
- `enums::NextState` is now `netty::handshake::NextState`
- Fixed some parsing bugs in the `generalized` submodule relating to improper use of `[V; N]` syntax
- Fixed a parsing bug with `UUID`s relating to the improper use of `[V; N]` syntax

## 0.19.0

- NBT string tags are now converted to valid "normal" UTF-8 strings
- Fixed some NBT parsing bugs ([#1](https://www.github.com/thisjaiden/golden_apple/issues/1))
- Added some NBT unit tests
- Updated `serde` and `serde_json`

## 0.18.0

- Updated to Minecraft 1.20.1 (protocol 767)
- Removed the Enchantments and Paintings enums (these are now data driven, and non-exaustive)

## 0.17.2

- Fixed username query API
- Updated dependencies
