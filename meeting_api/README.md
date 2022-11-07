## 启动

### 本地
ROCKET_ENV=dev cargo run
### 测试
ROCKET_ENV=stage cargo run
### 生产
ROCKET_ENV=prod cargo run

### 修改绑定的IP和端口
编辑Rocket.toml文件中的address和port字段