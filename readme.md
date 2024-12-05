# dspbptk

## 简介 · About

1. 基于对蓝图解析和Zopfli算法无损压缩蓝图
2. 把content转换为蓝图
3. TODO

---

1. Lossless compression of blueprints based on blueprint parsing and Zopfli algorithm
2. Convert content into blueprint
3. TODO

## 使用方法 · Usage

### 命令行 · CMD

```
Usage: dspbptk.exe [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input from file/dir. (*.txt *.content dir/)

Options:
  -o, --output <OUTPUT>  Output to file/dir
  -h, --help             Print help
  -V, --version          Print version
```

### 或者？· Any Else?

把文件或文件夹拖到`dspbptk.exe`上面（自动识别文件类型）  
Drag the file/directory onto the `dspbptk.exe` (automatically identify file types)  

## 注意 · Precautions

1. 如果不设置输出路径，默认将覆写原始蓝图
2. 输出文件时不会对比输入文件的体积，即使新的蓝图比老的更大

## 参考 Acknowledgements

* MD5f: https://github.com/Wesmania/dspbp