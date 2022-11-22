# intrative_admin-for-av-ws

## 用于音视频ws后端的权限控制
- 只监听 tcp 127.0.0.1 
- 一般不需要判断权限，这里是超级管理员才能调用的接口
- 

## 用法
- tcp server
```shell
cargo r admin
```
- admin with tcp client
```shell
cargo r
````

## 生产用法
- 拷贝admin代码到生产用ws后端代码中
- 参照dummy 实现 AsyncDbTrait
- ws 里 执行

```
toiki::spawn(admin::tcp::tcp_server());
```

- 新增子命令执行 
```r

admin::tcp::connect_tcp().await.unwrap();
if let Err(e) = crate::admin::admin().await {
   println!("error: {}", e);
}

```

## 环境变量

| env | 描述 | 
| --- | --- |
| ADMIN_PORT | TCP端口，默认9527  |
| LANG | 客户端 en or zh_CN  |
