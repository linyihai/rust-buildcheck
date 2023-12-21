# 构建规范检查工具

## 使用说明

命令行参数：

```shell
$ build-inspector.exe --help
Usage: build-inspector.exe [OPTIONS]

Options:
  -m, --manifest-path <MANIFEST_PATH>  the path of Cargo.toml in the root package directory [default: Cargo.toml]
  -c, --crate-size <CRATE_SIZE>        the packed crate size(MB) [default: 10]
  -o, --output-file <OUTPUT_FILE>      the output file, [default: rust_buildcheck_output.json]
  -h, --help                           Print help
  -V, --version                        Print version
```

- -m 指定Cargo.toml文件地址，不指定则默认为当前目录下Cargo.toml
- -o 指定结果输出文件，不指定默认为当前目录下的rust_buildcheck_output.json
- -c 执行crate包的大小检查项目，默认为10MB

在rust项目根目录下直接执行rust-bulidcheck二进制文件，输出实例:

```shell
$ build-inspector.exe 
package name: build-inspector
[G.RS.17] crate name `build-inspector` not start with ylong_ or huawei_ .
[G.RS.10] package `build-inspector` contains `license` field
the output file is rust_buildcheck_output.json
```
- **package name: build-inspector** 为检查Rust项目的包名
- G.RS.17为Rust构建规范对应条目
- `rust_buildcheck_output.json` 为最后输出的构建检查结果文件，可以通过`--out-file`参数自定义输出文件名
- 如果执行失败，返回错误码1(如shell中执行echo $? 返回1)


## 支持条目

- G.RS.05
- G.RS.06
- G.RS.08
- G.RS.10
- G.RS.17
- G.RS.18
- G.RS.20
  
