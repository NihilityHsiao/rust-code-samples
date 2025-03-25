# 说明
tonic实现负载均衡的案例,没有接入 服务注册中心
而是client直连server

运行方式(分别启动server和client)：
1. 在`tonic_grpc_load_balance`的**上级目录**运行下面两个命令:

`cargo run --bin tonic_grpc_load_balance_server`

`cargo run --bin tonic_grpc_load_balance_client`

在`client.rs`使用`tonic`的`Change`动态添加/删除 节点
在`server.rs`启动了多个rpc服务, 并且会输出是哪一个`server`收到了`client`的请求

