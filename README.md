# DSP Blueprint Toolkit

> 项目开发中，以下内容仅供参考

## 简介

这是一个蓝图解码/编码库，用于处理游戏[戴森球计划](https://store.steampowered.com/app/1366540/_/)中的工厂蓝图。

## 优点

1. 纯C语言编写，且代码经过深度优化，解析全球蓝图仅用时0.025秒。
2. 全缓冲，编解码的中间过程无需申请/使用额外的内存。
3. 多线程友好，每个编解码器使用的内存相互独立。

## 局限
1. 约100万或更多建筑的蓝图可能导致越界访问。这可以通过扩大缓冲区解决，但通常没有必要。因为即使是目前最高密度的全球白糖蓝图也只有约26万建筑。
2. 编码/解码过程只检查常见错误，只要md5f通过校验即信任蓝图。故意构造的恶意蓝图可能导致越界访问，或其它未定义行为。

## 开发计划

1. 计划支持Python API，具体实现还在探索中。

---

## 使用

1. 添加头文件
```C
#include "lib/libdspbptk.h"
```

2. 使用前需要先初始化编解码器
```C
dspbptk_coder_t coder;
dspbptk_init_coder(&coder);
```

3. 调用编码/解码函数
```C
blueprint_t blueprint;
blueprint_decode(&coder, &blueprint, string/*blueprint code*/);
// Edit blueprint here.
blueprint_encode(&coder, &blueprint, string_edited/*blueprint code edited*/);
```

4. 使用结束后必须释放编解码器和蓝图
```C
dspbptk_free_blueprint(&blueprint);
dspbptk_free_coder(&coder);
```