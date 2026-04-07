# A status information client for [river](https://github.com/riverwm/river)

![GitHub](https://img.shields.io/github/license/grvn/river-status?label=License&color=purple&logo=gitbook)
![Dependabot enabled](https://img.shields.io/badge/Dependabot-Enabled-brightgreen?logo=dependabot)
![GitHub contributors](https://img.shields.io/github/contributors/grvn/river-status?logo=github&label=Contributors)
![GitHub release (with filter)](https://img.shields.io/github/v/release/grvn/river-status?logo=github&label=Release&color=purple)
![GitHub issues](https://img.shields.io/github/issues-raw/grvn/river-status?label=Open%20Issues&logo=github&color=purple)
![GitHub closed issues](https://img.shields.io/github/issues-closed-raw/grvn/river-status?label=Closed%20Issues&logo=github&color=purple)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/grvn/river-status/main?label=Last%20Commit&logo=github&color=purple)

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

## Linting
```shell
cargo clippy --all-targets
```
