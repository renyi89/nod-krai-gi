# nod-krai-gi

[English Version](README.en.md) | [中文版本](README.md)

## Introduction

`nod-krai-gi` is forked from the [mavuika-rs](https://git.xeondev.com/mavuika-rs/mavuika-rs) project. Experimental implementation only.

## Feature Implementation/Differences
- `sdk-server`: Removed from the project, using hoyo-sdk (added dispatch redirection, because http was removed from the patch, only proxy can be used), set http_addr = "127.0.0.1:21000"
- `nod-krai-gi-database`: Using sqlite
- `nod-krai-gi-proto`: Modified to dynamically load protocol files
- `nod-krai-gi-ability`: Implemented ability system (incorrect implementation)
- `nod-krai-gi-muip-server`: Provides commands for debugging (incomplete, removed during compilation)

## Resource Requirements

### Protocol Files

Obtain protocol files from the [hk4e-protos](https://gitlab.com/kitkat-multiverse/genshin-protocol) repository and place them in the following location:

Some features are not supported, delete before use

```
assets/proto/VERSION/
```

Where `VERSION` is the game version number.

### Game Data

Obtain game data from the [AnimeGameData](https://gitlab.com/Dimbreath/AnimeGameData) repository and place it in the following locations:

```
assets/BinOutput/
assets/ExcelBinOutput/
```

## Acknowledgments

- Thanks to the [mavuika-rs](https://git.xeondev.com/mavuika-rs/mavuika-rs) project for providing the base code
- Thanks to the [hk4e-protos](https://gitlab.com/kitkat-multiverse/genshin-protocol) project for providing protocol files
- Thanks to the [AnimeGameData](https://gitlab.com/Dimbreath/AnimeGameData) project for providing game data
