# nod-krai-gi

[English Version](README.en.md) | [中文版本](README.md)

## 简介

`nod-krai-gi` 分叉自 [mavuika-rs](https://git.xeondev.com/mavuika-rs/mavuika-rs) 项目。仅实验性实现。


## 功能实现
- `nod-krai-gi-proto`: 修改为动态加载协议文件
- `nod-krai-gi-ability`: 实现能力系统（错误实现）
- `nod-krai-gi-muip-server`: 提供指令，用于调试。（未完成，编译时删除此部分）

## 资源要求

### 协议文件

从 [hk4e-protos](https://gitlab.com/kitkat-multiverse/genshin-protocol) 仓库获取协议文件，并放置在以下位置：
不支持某些特性，放之前删除

```
assets/proto/VERSION/
```

其中 `VERSION` 是游戏版本号。

### 游戏数据

从 [AnimeGameData](https://gitlab.com/Dimbreath/AnimeGameData) 仓库获取游戏数据，并放置在以下位置：

```
assets/BinOutput/
assets/ExcelBinOutput/
```


## 致谢

- 感谢 [mavuika-rs](https://git.xeondev.com/mavuika-rs/mavuika-rs) 项目提供的基础代码
- 感谢 [hk4e-protos](https://gitlab.com/kitkat-multiverse/genshin-protocol) 项目提供的协议文件
- 感谢 [AnimeGameData](https://gitlab.com/Dimbreath/AnimeGameData) 项目提供的游戏数据
