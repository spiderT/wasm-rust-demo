# Rust & WebAssembly

- [Rust \& WebAssembly](#rust--webassembly)
  - [项目介绍](#项目介绍)
    - [项目启动](#项目启动)
      - [构建Rust项目](#构建rust项目)
      - [web端启动](#web端启动)
  - [Rust](#rust)
    - [Rust 在前端领域的应用](#rust-在前端领域的应用)
    - [优秀的rust前端库](#优秀的rust前端库)
  - [WebAssembly](#webassembly)
    - [WASM Runtime](#wasm-runtime)
  - [编译 Rust 为 WebAssembly](#编译-rust-为-webassembly)
    - [安装 Rust 环境](#安装-rust-环境)
    - [创建一个新的 Rust 包](#创建一个新的-rust-包)
    - [使用 wasm-bindgen 在 Rust 与 JavaScript 之间通信](#使用-wasm-bindgen-在-rust-与-javascript-之间通信)
    - [在 Rust 中调用来自 JavaScript 的外部函数](#在-rust-中调用来自-javascript-的外部函数)
    - [配置 Cargo.toml](#配置-cargotoml)
    - [构建包](#构建包)
  - [缩小 .wasm 大小](#缩小-wasm-大小)
    - [使用链接时间优化 (LTO) 进行编译](#使用链接时间优化-lto-进行编译)
    - [告诉 LLVM 优化大小而不是速度](#告诉-llvm-优化大小而不是速度)
    - [使用wasm-opt工具](#使用wasm-opt工具)
    - [代码大小分析器](#代码大小分析器)
    - [手动检查 LLVM-IR](#手动检查-llvm-ir)

## 项目介绍

用Rust处理图片，交换R/G/B 颜色通道改变颜色，修改R/G/B 透明度达到滤镜效果，利用wasm-pack将Rust 编译成WebAssembly。参考资料：https://github.com/silvia-odwyer/photon.

![demo](./images/1.jpg)

### 项目启动

#### 构建Rust项目

用wasm-pack，是根据以下构建步骤:

- Rust 1.30 或打上版本，通过rustup安装wasm32-unknown-unknown目标(target),
- 用cargo将 Rust 的源代码编译为 WebAssembly 的 .wasm二进制文件,
- 为 Rust 生成的 WebAssembly 使用wasm-bindgen，生成 JavaScript API.
  
wasm-pack 构建 rust 生成的 WebAssembly 包

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

要完成所有这些操作，请在项目目录中，运行此命令:

```bash
cd crate
wasm-pack build
```

构建完成后，我们可以在 pkg 目录中找到它的工件，它应该包含以下内容：

```bash
├── README.md
├── package.json
├── wasm_rust_demo_rs.d.ts
├── wasm_rust_demo_rs.js
├── wasm_rust_demo_rs_bg.js
├── wasm_rust_demo_rs_bg.wasm
└── wasm_rust_demo_rs_bg.wasm.d.ts
```

#### web端启动

```bash
cd www
npm install
npm start
```

## Rust

在 Stack Overflow 的年度开发者调查中，Rust 连续第八年位居“最受欢迎的编程语言”榜首。具有以下优势：

1. 严格的编译时类型检查：Rust的类型系统可以在编译时捕获许多常见的错误，如空指针引用和数据竞争，从而提供更高的代码安全性。
2. 所有权模型和借用检查器：Rust的所有权模型允许在编译时跟踪和管理内存的使用，避免了内存泄漏和悬挂指针等问题。
3. 并发性：Rust提供了轻量级的并发，如线程和消息传递机制，使开发人员能够更好地处理并发任务。
4. 高性能：Rust的零成本抽象和对底层硬件的精细控制，使其在处理计算密集型任务和优化性能方面表现出色。

### Rust 在前端领域的应用

Rust在前端开发中有多种应用场景，包括但不限于：

> 图像处理
Rust的高性能和安全性使其成为处理图像处理库的理想选择。通过使用Rust编写的图像处理库，可以实现高效的图像处理和编辑功能。
> 密码学
由于Rust对安全性的重视，它在密码学领域得到广泛应用。开发人员可以使用Rust编写密码学库，提供安全的加密算法和数据保护功能。
> 网络服务器
Rust的并发性和性能使其成为构建高性能网络服务器的理想选择。开发人员可以使用Rust编写可扩展的服务器端应用程序，提供稳定和高效的网络服务。

### 优秀的rust前端库

Rust 在前端构建工具中的应用

> Deno
Github: https://github.com/denoland/deno  
是一个 JavaScript、TypeScript 和 WebAssembly 运行时，具有安全的默认设置和出色的开发人员体验。它基于 V8、Rust 和 Tokio 构建。

> SWC
Github: https://github.com/swc-project/swc
SWC（代表 Speedy Web Compiler）是一个用 Rust 编写的超快速 TypeScript / JavaScript 编译器。SWC 是一个可扩展的基于 Rust 的前端构建工具，目前核心功能相当于 Babel, SWC 在单线程上比 Babel 快 20 倍，在四核上快 70 倍。

> Parcel
Github: https://github.com/parcel-bundler/parcel
支持以 HTML 作为入口的零配置构建工具，Parcel 支持多种开箱即用的语言和文件类型，从 HTML、CSS 和 JavaScript 等 Web 技术到图像、字体、视频等资产。当您使用默认不包含的文件类型时，Parcel 将自动为您安装所有必要的插件和开发依赖项。Parcel 的 JavaScript 编译器和源映射是建立在 SWC 编译器之上的，在 SWC 之上，Parcel 实现了依赖项收集、捆绑、摇树优化、热重载等。

> Yew
Github: https://github.com/yewstack/yew
Yew 是一个现代 Rust 框架，用于使用 WebAssembly 创建多线程前端 Web 应用程序。

> Rspack
Github: https://github.com/web-infra-dev/rspack
基于 Rust 的高性能模块打包工具.

## WebAssembly

WebAssembly 是一门低级的类汇编语言。它有一种紧凑的二进制格式，使其能够以接近原生性能的速度运行，并且为诸如 C++ 和 Rust 等拥有低级的内存模型语言提供了一个编译目标以便它们能够在网络上运行。

Rust与WebAssembly（Wasm）相结合，为前端开发带来了许多好处：

1. 更快的加载速度：通过将Rust代码编译为WebAssembly模块，在浏览器中执行原生代码，可以显著提高应用程序的加载速度。
2. 更高的计算能力：Rust的高性能和底层控制特性使得在浏览器中运行复杂的计算任务成为可能，从而提供更丰富的功能和用户体验。
3. 与JavaScript的互操作性：Rust可以与JavaScript无缝互操作，通过外部函数接口（FFI）调用JavaScript函数，实现跨语言的开发和集成。

### WASM Runtime

WASM 在浏览器里面运行的架构

![WebAssembly](./images/wasm1.png)

## 编译 Rust 为 WebAssembly

将编译 Rust 项目为 wasm 并在一个 web 应用中使用它。

### 安装 Rust 环境

> 安装 Rust

文档：https://www.rust-lang.org/tools/install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> wasm-pack

把rust代码编译成 WebAssembly 并制造出正确的 npm 包。

```bash
cargo install wasm-pack
```

### 创建一个新的 Rust 包

```bash
cargo new --lib hello-wasm
```

项目结构

```text
.
├── Cargo.toml
└── src
    └── lib.rs
```

### 使用 wasm-bindgen 在 Rust 与 JavaScript 之间通信

wasm-pack 使用另一个工具 wasm-bindgen 来提供 JavaScript 和 Rust 类型之间的桥梁。它允许 JavaScript 使用字符串调用 Rust API，或调用 Rust 函数来捕获 JavaScript 异常。

```rust
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
```

### 在 Rust 中调用来自 JavaScript 的外部函数

```rust
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
```

### 配置 Cargo.toml

添加底下的部分

```toml
[dependencies]
wasm-bindgen = "0.2"

[lib]
crate-type = ["cdylib"]
```

> [lib] — 告诉 Rust 为我们的包建立一个 cdylib 版本, --crate-type=cdylib 或 #[crate_type = "cdylib"] - 将生成一个动态系统库。如果编译输出的动态库要被另一种语言加载使用，请使用这种编译选项。详细配置项，见：https://www.rustwiki.org.cn/zh-CN/reference/linkage.html#%E9%93%BE%E6%8E%A5。

> [dependencies] - 告诉 Cargo 需要依赖哪个版本的 wasm-bindgen。  

### 构建包

```bash
wasm-pack build
```

wasm-pack build 将做以下几件事：

- 将你的 Rust 代码编译成 WebAssembly。
- 在编译好的 WebAssembly 代码基础上运行 wasm-bindgen，生成一个 JavaScript 文件将 WebAssembly 文件包装成一个模块以便 npm 能够识别它。
- 创建一个 pkg 文件夹并将 JavaScript 文件和生成的 WebAssembly 代码移到其中。
- 读取你的 Cargo.toml 并生成相应的 package.json。
- 复制你的 README.md (如果有的话) 到文件夹中。

```text
.
├── README.md
├── hello_wasm.d.ts
├── hello_wasm.js
├── hello_wasm_bg.js
├── hello_wasm_bg.wasm
├── hello_wasm_bg.wasm.d.ts
└── package.json
```

## 缩小 .wasm 大小

参考：https://rustwasm.github.io/book/game-of-life/code-size.html#shrinking-wasm-size

使用很多配置选项来让rustc缩小.wasm二进制文件。在某些情况下，我们的编译时间较长，.wasm就越小。另一方面说，我们以较小的代码大小交换 WebAssembly 的运行时速度。我们应该权衡每个选项，并且在我们用运行时速度交换代码大小时，应分析和度量，以便做出关于此次交易是否值得的明智决策。

### 使用链接时间优化 (LTO) 进行编译

在Cargo.toml，添加lto = true在[profile.release]部分:

```toml
[profile.release]
lto = true
```

这为 LLVM 提供了更多内联和修剪功能的机会. 它不仅会成功.wasm更小，但它也会在运行时更快! 缺点是编译需要更长时间.

### 告诉 LLVM 优化大小而不是速度

默认情况下，调整 LLVM 的优化过程是提高速度，而不是大小。 我们可以通过修改目标， 来将目标更改为代码大小

[profile.release]部分:

```toml
[profile.release]
opt-level = 's'
```

或者，更进一步优化尺寸，以更大的速度成本:

```toml
[profile.release]
opt-level = 'z'
```

### 使用wasm-opt工具

该工具套件是特定于 WebAssembly 的编译器工具的集合。使用wasm-opt后，处理 LLVM 生成的.wasm二进制文件通常可以节省 15-20%的代码大小，https://docs.rs/wasm-opt/latest/wasm_opt。

```bash
wasm-opt -Oz -o output.wasm input.wasm
```

wasm-opt的选项一共有

- -Os or -O  执行默认的优化过程，重点关注代码大小
- -Oz  专注于代码大小, 优化体积最明显
- -O0  不执行优化过程
- -O1  execute-O1优化过程（快速而有用的选择，对迭代构建有用）
- -O2  execute-O2优化通过（大多数选择，通常获得最多性能）
- -O3  执行-O3优化过程（可能会花费大量时间进行优化）
- -O4  execute-O4优化过程（也可以使IR变平，这可能需要更多的时间和内存，但对嵌套/复杂/优化程度较低的输入很有用）

以本项目为例，构建尺寸如下表

| 选项 | size(kb) |
|-----|----------|
| -Os |   64.3   |
| -O  |   64.3   |
| -Oz |   63.3   |
| -O0 |   72.0   |
| -O1 |   70.1   |
| -O2 |   66.4   |
| -O3 |   65.7   |
| -O4 |   65.8   |

### 代码大小分析器

Twiggy 是 Wasm 的代码大小分析器。它分析二进制文件的调用图。 https://rustwasm.github.io/twiggy。

用法

```bash
twiggy top <input> -n <max-items> -o <output-destination> --format <output-format> --mode <parse-mode>
```

### 手动检查 LLVM-IR

LLVM-IR 是 LLVM 生成 WebAssembly 之前编译器工具链中的最终中间表示。 因此,它与最终发出的 WebAssembly 非常相似。 更多 LLVM-IR 通常意味着更多.wasm大小,如果一个函数占 LLVM-IR 的 25%,那么它通常会占 25%.wasm。 虽然这些数字一般只保留。 LLVM-IR 具有关键信息，而这些信息并不存在.wasm中 (因为 WebAssembly 缺少像 DWARF 这样的调试格式) : 哪些子程序被内联到 给定的函数中。

使用此方法生成 LLVM-IR:

```bash
cargo rustc --release -- --emit llvm-ir
```







参考资料：  
Compiling from Rust to WebAssembly: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm  
Rust and WebAssembly: https://llever.com/rustwasm-book/introduction.zh.html  
wasm-bindgen: https://rustwasm.wasmdev.cn/docs/wasm-bindgen/introduction.html
