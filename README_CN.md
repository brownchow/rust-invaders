新的 YouTube 完整教程 - [Rust Bevy - 完整教程 - 游戏开发](https://www.youtube.com/watch?v=j7qHwb7geIM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

Rust [Bevy](https://bevyengine.org/) - 游戏开发教程

### 最新 Bevy 版本更新

- `2024-11-29` - 更新到 Bevy 0.15。感谢 [Matt Woelfel](https://github.com/woelfman)
- `2024-06-08` - 更新到 Bevy 0.14。感谢 [Matt Woelfel](https://github.com/woelfman)
- `2024-05-23` - 更新到 Bevy 0.12 & 0.13。感谢 [Vinzenz Schroeter (@kaesaecracker)](https://github.com/kaesaecracker)
- `2023-10-11` - 更新到 Bevy 0.11.3。感谢 [juliohq](https://github.com/juliohq)
- `2023-05-02` - 更新到 Bevy 0.10。感谢 [ehasnain](https://github.com/ehasnain)
- `2022-12-13` - 更新到 Bevy 0.9。感谢 [GiulianoCTRL](https://github.com/GiulianoCTRL)
- `2022-08-07` - 主分支更新到 Bevy 0.8。感谢 [DomagojRatko](https://github.com/DomagojRatko)

### MacOS 设置

- 确保 [Rust 和 Cargo 已安装](https://www.rust-lang.org/tools/install)
- 删除 Cargo.lock，否则首次运行可能会出错：`rm Cargo.lock`
- 使用 [Homebrew](https://brew.sh/) 安装 Cmake：`brew install cmake`
- 安装 Cargo Watch：`cargo install cargo-watch`

### Fedora (Linux) 要求

确保使用您选择的包管理器在系统上安装以下包

```
dnf install rust-alsa-sys-devel
dnf install rust-libudev-devel
```
感谢 [janpauldahlke](https://github.com/janpauldahlke)

### 浏览器 (WebAssembly) - Gitpod

<a href="https://gitpod.io/#https://github.com/mesquitaoliveira/rust-invaders.git" ><img src="https://gitpod.io/button/open-in-gitpod.svg" alt="在 Gitpod 中打开" /><a>


### 开发

快速开发：
- `cargo run --features bevy/dynamic_linking`
- `cargo watch -q -c -x 'run --features bevy/dynamic_linking'`

wsl ubuntu22.04:
- ` WGPU_BACKEND=vulkan cargo run --features bevy/dynamic_linking`

- 其他 Rust 视频：
    - [Jeremy Chone](https://www.youtube.com/jeremychone) 频道的每周 Rust 视频
    - [Rust 游戏开发教程](https://youtube.com/playlist?list=PL7r-PXl6ZPcCB_9zZFU0krBoGK3y5f5Vt)

### 变更日志

- 2022-12-13 - 更新到 Bevy 0.9。感谢 [GiulianoCTRL](https://github.com/GiulianoCTRL)
- 2022-08-07 - 主分支更新到 Bevy 0.8。感谢 [DomagojRatko](https://github.com/DomagojRatko)
- 2022-08-07 - 主分支更新到 Bevy 0.8 感谢 [@DomagojRatko](https://github.com/DomagojRatko)
- 2022-05-09 - 更新到 v0.7 的新教程。请参阅 [Rust Bevy - 完整教程 - 游戏开发](https://www.youtube.com/watch?v=j7qHwb7geIM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
- 2022-01-28 - 代码已更新到 **Bevy v0.6**！！！
- 2021-06-25 - 初始版本（适用于旧教程 [第 1 集](https://youtu.be/Yb3vInxzKGE)，[第 2 集](https://youtu.be/Dl4PJG0eRhg)，[第 3 集](https://youtu.be/4nEUX2hf2ZI)）


## Rust & Bevy & 其他资源

学习和使用的主题：

- 游戏开发的 Rust 编程
- 游戏 ECS 引擎 Bevy
- Bevy 系统、Bevy 组件、Bevy 资源
- Bevy 插件
- Bevy 实体的生成和销毁
- Bevy SpriteBundle、精灵表 (SpriteAtlas)
- Bevy 时间步长、时间和计时器
- Bevy 自定义系统条件
- 生成爆炸的间接策略
- 用于精灵动画的精灵表图集
- 基本 Rust 编程
- Rust 模块
- Rust 闭包
- Rust 匹配

资源：

- Rust：
    - [Rust 书籍](https://doc.rust-lang.org/book/)
    - [Rust 视频课程](https://www.youtube.com/playlist?list=PL7r-PXl6ZPcB4jn1_VR3D8tSK9DxOaiQE)
- Bevy：
    - [Bevy 官方网站](https://bevyengine.org/)
    - [Bevy 官方书籍](https://bevyengine.org/learn/book/introduction/)
    - [非官方 Bevy 速查手册](https://bevy-cheatbook.github.io/)
    - [Bevy 官方 API 文档](https://docs.rs/bevy/latest/bevy/index.html)
    - [Bevy 官方资源](https://bevyengine.org/assets/)
    - [官方 GitHub 示例](https://github.com/bevyengine/bevy/tree/latest/examples)
    - [优秀博客文章 - 贪吃蛇游戏](https://mbuffett.com/posts/bevy-snake-tutorial/)
- 资源：
    - [玩家、激光、敌人精灵](https://opengameart.org/content/space-shooter-redux)
    - [爆炸](https://opengameart.org/content/explosion)


<br /><br /><br />
[此仓库](https://github.com/jeremychone-channel/rust-invaders)