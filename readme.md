# dspbptk

## 简介 · About

安全、高效的蓝图解析库，用于游戏《戴森球计划》

---

## 功能 · Function

1. 基于建筑排序和Zopfli算法无损压缩蓝图
2. 在content和txt两种蓝图格式之间转换
3. TODO 常用的蓝图编辑工具

---

1. Lossless compression of blueprints based on buildings sort and Zopfli algorithm
2. Convert between content / blueprint
3. TODO Useful toolkit for blueprint edit

## 注意 · Warning

1. 如果不设置输出路径，默认将覆写原始蓝图
2. 输出文件时不会对比输入文件的体积，即使新的蓝图比老的更大

## 使用方法 · Usage

### 命令行 · CMD

```
Usage: dspbptk.exe [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input from file/dir. (*.txt *.content dir/)

Options:
  -o, --output <OUTPUT>
          Output to file/dir. (*.txt dir/)
  -f, --filetype <FILETYPE>
          Output type: txt, content [default: txt]
  -a, --actions [<ACTIONS>...]
          Actions of edit blueprint
  -s, --sort-buildings
          Sort buildings for smaller blueprint
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