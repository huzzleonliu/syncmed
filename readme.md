# leptos全栈应用模板

##
这是一个leptos-ssr的使用模板，用于方便地创建leptos-ssr网页

## 准备工作
- 安装leptos需要的组件
    - ```cargo install cargo-leptos```
    - ```rustup target add wasm32-unknown-unknown```
- 安装pglib库
    - ```paru -S postgresql```

- 安装diesel-cli
    - ```cargo install diesel_cli --no-default-features --features postgres```针对postgres数据库进行安装

- 安装nodejs
    - ```paru -S nodejs```
- 安装cargo-generate
    - ```cargo install cargo-generate```

## 部署
- 使用```cargo generate --git <git 仓库> --branch <branch 名>```配置项目
    - 参考部署命令```cargo generate --git ssh://git@forgejo.huzzleonliu.com:20504/huzzleonliu/leptos-ssr-example.git```
- 配置后，进入syncmed目录，并执行
    - ```cargo leptos build --release```编译ssr网页，否则podman-compose无法部署
    - ```npm install```可选，因为部署过程中应该已经完成了
- 之后返回根目录
    - ```podman-compose up -d```运行容器

- 容器运行后进入syncmed目录，并执行
    - ```diesel migration run```导入数据库表格结构


## 技术栈

- 网站
    - leptos-ssr
    - axum
    - tailwind
    - daisyui

    - diesel

- 数据库
    - postgresql:latest
- 打包
    - cargo-generate
- 容器
    - podman
    - podman-compose

## 下一步工作
- 在generate选项中加入是否默认添加案例
- 在generate选项中加入是否使用daisyUI
- 添加技术栈
  - leptos-icon
  - redis
