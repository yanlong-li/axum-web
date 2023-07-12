# Axum web framework tutorial

本项目主要是学会“用”，而非“造”

## 包含内容
* mysql
* redis
* cookie


### 目录结构
> 没找到比较好的项目用于规范目录结构定义，下面是我在学习过程中的自定义，有些可能不能准确别到其作用，仅供学习参考
    
    .
    ├─assets            资源目录
    │  ├─locales        本地化文本
    │  └─web            公共可访问web资源
    ├─http              api 测试脚本
    ├─src
    │  ├─controllers    处理控制器集合
    │  │  ├─login       登录
    │  │  ├─status      服务状态
    │  │  ├─test        一些临时测试的代码
    │  │  ├─users       用户管理相关逻辑
    │  │  └─ws          websocket 连接
    │  ├─databases      数据库连接
    │  ├─middlewares    中间件
    │  ├─models         交互模型
    │  │  ├─auth        认证模型
    │  │  └─users       用户模型
    │  ├─routes         路由
    │  ├─schema         数据库结构
    │  ├─services       中间服务 用于连接 controller 和 schema
    │  └─utils          其他工具类
    │      ├─i18n       国际化支持
    │      └─response   api响应内容封装
    ├─mysql.sql         数据库sql


### crate 依赖

目前主要是想实现 mysql、redis 等交互，所以依赖以下 crates

* sqlx
* redis

除了所必须的依赖外，你可以替换成你所熟悉的库，如: r2d2、Diesel等

甚至你也可以换掉 axum ，比如 actix-web

这都取决于你自己，这是 axum web 的学习项目