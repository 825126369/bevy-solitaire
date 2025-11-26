<h1>Rust游戏面试作品</h1>

1: Rust 语言 开发 小游戏

2: bevy引擎官网: https://github.com/bevyengine/bevy

<h1>Rust基础知识</h1>

1: VSCode 终端: cargo new hello_rust 会创建一个hello_rust工程。

2： Cargo.toml 相当于 VS工程的工程文件，用来管理配置这个工程的。

3: 运行: cargo run --release

4: crates.io 是 Rust 的官方公共包注册中心.

5： #[cfg(not(target_arch = "wasm32"))] 是 条件编译属性，告诉编译器：「只在非 WASM 架构时才编译下面的代码」。
