# DatabaseMysqlConn-rust
> https://docs.rs/mysql/24.0.0/mysql/

## 创建连接

[Connection](./examples/conn.rs)

## 创建数据表 query_drop

[Create_table](./examples/create_table.rs)

## 写操作
- 插入单条数据 conn.exec_drop()
- 插入批量数据 conn.exec_batch()
- 预编译语句插入大量数据， conn.prep()
- 获取主键 conn.last_insert_id()
- 更新 conn.prep
- 删除 conn.exec_drop

### 插入
单条插入

[Insert_Single](./examples/insert_single.rs)

批量插入
，首先定义插入数据的结构体,
通过数组存储结构体数据，执行批量插入

[Insert_batch](./examples/insert_batch.rs)

预编译语句插入大量数据， conn.prep() + conn.exec_drop() 

不会判重，重复执行的话会插入多条同样的数据

[Pre_Insert_Single](./examples/insert_pre.rs)

- 获取主键 conn.last_insert_id()
```rust
println!("新插入的记录的主键为: {}", conn.last_insert_id())
```
更新 conn.prep()

[Update](./examples/update.rs)

删除 conn.exec_drop()

[Delete](./examples/delete.rs)

## 读操作
查询：query_iter + query_map

[Query](./examples/query.rs)

获取单条数据： query_first
查询特定数据行，可能会出现下面几种情况:

- 找到，返回实际数据
- 没有找到行
- 发生错误
所以，使用query_first函数返回的是Option的结果。 需要将其解包两次才可以获取实际的行数据:

条件查询： exec_first

[QueryFirst](./examples/query_first.rs)
