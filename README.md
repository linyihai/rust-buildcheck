# 构建规范检查工具

## 使用说明

命令行参数：

```shell
$ rust-buildcheck.exe --help
Usage: rust-buildcheck.exe [OPTIONS]

Options:
  -m, --manifest-path <MANIFEST_PATH>  the path of Cargo.toml in the root package directory [default: Cargo.toml]
  -c, --crate-size <CRATE_SIZE>        the packed crate size(MB) [default: 10]
  -h, --help                           Print help
  -V, --version                        Print version
```

- -m 指定Cargo.toml文件地址，不指定则默认为当前目录下Cargo.toml
- -c 执行crate包的大小检查项目，默认为10MB

在rust项目根目录下直接执行rust-bulidcheck二进制文件，输出实例:

```shell
$ rust-buildcheck.exe
package name: _clippytest
build check failed: [G.RS.18] crate name `_clippytest` not start with ylong_ or huawei_ .
build check failed: [G.RS.05] E:\rust\clippytest\Cargo.toml has no edition field, please add a edition like `edition = 2021`.
build check failed: [G.RS.06] E:\rust\clippytest\Cargo.toml no newest rust editon.
build check failed: [G.RS.19] package _clippytest use no explicit version for dependency regex.
```

其中G.RS.18为Rust构建规范对应条目

如果执行失败，返回错误码1

```shell
$ echo $?
1
```

## 支持条目

- G.RS.05
- G.RS.06
- G.RS.08
- G.RS.14
- G.RS.18
- G.RS.19
- G.RS.21
  

