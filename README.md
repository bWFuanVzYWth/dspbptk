# DSP Blueprint Toolkit

> 项目开发中，以下内容仅供参考

## 简介

这是一个蓝图解码/编码库，用于处理游戏[戴森球计划](https://store.steampowered.com/app/1366540/_/)中的工厂蓝图。

## 特点

1. 纯C语言编写，且代码经过高度优化，解析全球蓝图仅用时0.025秒(游戏里点一下得卡好几秒)
2. 全缓冲，编解码过程除了blueprint_t内部无动态内存分配
3. 多线程友好，每个编解码器有独立的内存空间

## 开发计划

1. 计划支持Python API，具体实现还在探索中

---

## 使用

1. 添加头文件
```C
#include "lib/libdspbptk.h"
```

1. 使用前需要先初始化编解码器
```C
dspbptk_coder_t coder;
dspbptk_init_coder(&coder);
```

1. 调用编码/解码函数。
```C
blueprint_t blueprint;
blueprint_decode(&coder, &blueprint, string/*blueprint code*/);
// Edit blueprint here.
blueprint_encode(&coder, &blueprint, string_edited/*blueprint code edited*/);
```

1. 使用结束后必须释放编解码器和蓝图
```C
dspbptk_free_blueprint(&blueprint);
dspbptk_free_coder(&coder);
```