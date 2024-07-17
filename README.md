# dev-tools

该项目是是`rust + vue`开发的桌面端项目，是前端可视化开发工具，当然此开发工具不同于平常的开发工具，是有利于自己的一款工具；

<font color="#ff0000">目前属于测试阶段，有些问题还未发现；</font>

## 背景

在开发中，我曾经有个疑问，就是不论在`linux` 还是`docker`的开发环境下，它都是有专属的可视化面板的，比如`宝塔，1panel, docker桌面端`等等；就好比说您如果不清楚操作命令， 那么您就可以使用面板的简单操作；

在这样的想法下， 于是我就简单的开发，之前的一个版本是`electron + vue`的开发，但是在开发过程中，遇到的问题太多太多了，`electron`操作的是`node`；于是就开发了一半； 不论是在开发过程中，还是最后的打包部署其体积都是很大的；简单的说就是底层用的是`electron`，其外层就是`webview`页面，不做优化的情况下体积很大；一个简单的工具总不是几十兆的大小吧，不合适；

于是就简单的接触了`rust`,

## 开发进度

### <Badge type="info" text="已完成" />

- 基础页面功能已完成;
- 主题切换功能已完成；
- git 配置功能已完成；
- NODE 管理功能已完成；(部分情况下出现闪退问题)

### <Badge type="danger" text="未完成" />

- 代码需要进行优化整理；
- 操作系统的兜底
- 日志打印

### <Badge type="warning" text="后续添加的功能" />

- 添加软件更新功能
- 添加跟更新日志

## 有哪些功能

1. **欢迎页：** 介绍本款软件的作用，以及简单介绍；
2. **首页：** 涵盖前端常用的开发工具
3. **Node 管理：** 可视化的 node 管理，可以查看 node 的版本，以及安装卸载 node；同时也可操作 npm 下载的全局包；
4. **Git 配置：** 全局的 git 配置

## 亮点有哪些

1. 如果您想要切换 node 版本，那么您可以使用命令行`nvm use 16.14.0`或者`fnm use 16.14.0`；但是您想要在可视化界面上切换，那么您需要在相对应的界面进行操作；
2. 您不需要低版本的 node，因为您只需要在`node`的版本上进行操作；当然携带了安装依赖的功能，您可以对于全局依赖进行管理

## 图片

![欢迎页面](https://file.wangzevw.com/images/image.3k7wazx9ha.webp)

![首页](https://file.wangzevw.com/images/image.lyl6vzj1.webp)

![Node管理](https://file.wangzevw.com/images/image.4n7llvv3zq.webp)

![安装Node](https://file.wangzevw.com/images/image.231r98vwy8.webp)

![已经安装的Node](https://file.wangzevw.com/images/image.6pne9xv39s.webp)

![镜像源](https://file.wangzevw.com/images/image.32humf03cq.webp)

![安装全局包](https://file.wangzevw.com/images/image.5fkh3mew3s.webp)

![git](https://file.wangzevw.com/images/image.7sn3ktth5u.webp)

![主题设置](https://file.wangzevw.com/images/image.lyl71441.webp)

![菜单模式](https://file.wangzevw.com/images/image.2rv0t9ntpo.webp)
