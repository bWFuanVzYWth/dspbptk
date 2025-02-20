<!-- TODO 更新readme，简介已经过时 -->
# dspbptk

## 简介 · About

《戴森球计划》中的蓝图工具集

* 内存安全的蓝图解析/编码库，可以在0.03s内解析一张18万建筑的全球白糖蓝图
* 常用的蓝图处理相关工具集，如线性变换等
* 常用的算法生成蓝图相关工具集，如偏移密铺等

---

# dspbptk.exe

## 简介 · About

* 基于此框架开发的APP

## 注意 · Warning

* 如果不设置输出路径，默认将覆写原始蓝图
* 输出文件时不会对比输入文件的体积，即使新的蓝图比老的更大

## 使用方法 · Usage

### 命令行 · CMD

```
Usage: dspbptk.exe [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input from file/dir. (*.txt *.content dir/)

Options:
  -o, --output <OUTPUT>
          Output to file/dir. (*.* dir/)
  -t, --type-output <TYPE_OUTPUT>
          Output type: txt, content [default: txt]
  -r, --rounding-local-offset
          Round local_offset to 1/300 may make blueprint smaller. Lossy
      --no-sorting-buildings
          Sorting buildings may make blueprint smaller. Lossless
      --iteration-count <ITERATION_COUNT>
          Compress arguments: zopfli iteration_count [default: 256]
      --iterations-without-improvement <ITERATIONS_WITHOUT_IMPROVEMENT>
          Compress arguments: zopfli iterations_without_improvement [default: 18446744073709551615]
      --maximum-block-splits <MAXIMUM_BLOCK_SPLITS>
          Compress arguments: zopfli maximum_block_splits [default: 0]
  -h, --help
          Print help
  -V, --version
          Print version
```

### 或者？· Any Else?

把文件或文件夹拖到`dspbptk.exe`上面（自动识别文件类型）  
Drag the file/directory onto the `dspbptk.exe` (automatically identify file types)  

## 参考 Acknowledgements

* MD5f: https://github.com/Wesmania/dspbp

