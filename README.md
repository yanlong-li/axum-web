# Axum web framework tutorial

本项目主要是学会“用”，而非“造”


目录结构参考： https://github.com/rust10x/rust-web-app


目前包含：
* MySQL
* Redis
* Session
* Cookie
* WebSocket
* GeoIP2
* reqwest


业务逻辑包含用户登录、用户登录状态判断（Session）、IP信息查询、网络请求（模拟请求第三方api接口）、WebSocket连接


start dev services
```shell
docker compose -f docker-compose.yaml up -d
```

stop dev services

```shell
docker compose -f docker-compose.yaml down
```