# 构建规范检查工具

## 使用说明

命令行参数：

```shell
$ build-inspector[.exe] --help
Usage: build-inspector [OPTIONS]

Options:
  -m, --manifest-path <MANIFEST_PATH>  The path of Cargo.toml in the root package directory [default: Cargo.toml]
  -c, --crate-size <CRATE_SIZE>        The packed crate size(MB) [default: 10]
  -o, --output-file <OUTPUT_FILE>      The output file, [default: rust_buildcheck_output.json]
      --offline                        Run without accessing the network
  -h, --help                           Print help
  -V, --version                        Print version
```

- -m 指定Cargo.toml文件地址，不指定则默认为当前目录下Cargo.toml
- -o 指定结果输出文件，不指定默认为当前目录下的rust_buildcheck_output.json
- -c 执行crate包的大小检查项目，默认为10MB
- --offline 支持无网络访问执行

在rust项目根目录下直接执行rust-bulidcheck二进制文件，输出实例:

```shell
$ build-inspector[.exe]
package name: build
crate name `build-inspector` not start with ylong_ or huawei_ .
package `build-inspector` contains `license` field
the output file is rust_buildcheck_output.json
```

- **package name: build-inspector** 为检查Rust项目的包名
- `rust_buildcheck_output.json` 为最后输出的构建检查结果文件，可以通过`--out-file`参数自定义输出文件名
- 如果执行失败，返回错误码1(如shell中执行echo $? 返回1)

## 支持条目

- G.RS.05
- G.RS.06
- G.RS.08
- G.RS.10
- G.RS.14
- G.RS.17
- G.RS.18
- G.RS.20

## 注意事项  

- 工具依赖于`cargo_metadata`获取rust项目信息，要求运行本工具系统上安装rust工具链，见[Rust内源安装工具链](https://openx.huawei.com/communityHome/postDetail?postId=2688&id=90)
- 默认运行需要访问依赖源，请确保网络能正常访问内源mirror或者crates-io; 如无法访问依赖源，请通过`--offline`参数执行
- 注意glibc版本问题，高版本glibc构建出来的二进制在低版本上运行不了，如默认情况ubuntu22(glibc 2.35)构建的rust二进制无法在ubuntu18(glibc 2.27)执行。

常见Linux发行版glibc版本：

|  Distro   |  glibc  |
|   ---     |   ---   |
| CentOS 7  |  2.17   |
| CentOS 8  |  2.28   |
| Debian 9  |  2.24   |
| Debian 10 |  2.28   |
| Debian 11 |  2.31   |
| Ubuntu 16.04 LTS | 2.23 |
| Ubuntu 18.04 LTS | 2.27 |
| Ubuntu 20.04 LTS | 2.31 |
| Ubuntu 22.04 LTS | 2.35 |

更多[glibc版本信息](https://repology.org/project/glibc/versions)
