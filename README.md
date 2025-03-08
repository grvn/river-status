# A status information client for [river](https://github.com/riverwm/river)

This client is designed to be used with [yambar](https://codeberg.org/dnkl/yambar) or [eww](https://github.com/elkowar/eww) or any other thing that could benefit from status information about tags, windows etc. from river.

## Building

### Dependencies
- rust

> *__NOTE__*: This repo uses [river/protocol](https://github.com/riverwm/river/tree/master/protocol) as a submodule.
```shell
git clone --recursive git@github.com:grvn/river-status.git
cd river-status
cargo build --release
```