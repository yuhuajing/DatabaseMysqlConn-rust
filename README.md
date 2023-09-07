# DatabaseMysqlConn-rust
> https://docs.rs/mysql/24.0.0/mysql/

## 创建链接
```rust
use mysql::prelude::*;
use mysql::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mysql_url: &str = "mysql://root:123456@localhost:3306/testUser";
    let pool = Pool::new(mysql_url)?;
    let mut conn = pool.get_conn()?;
}
```

## 创建数据表 query_drop
```rust
    conn.query_drop(
        r"CREATE TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text)",
    )?;
```

## 写操作
- 插入单条数据 conn.exec_drop()
- 插入批量数据 conn.exec_batch()
- 预编译语句插入大量数据， conn.prep()
- 获取主键 conn.last_insert_id()
- 更新 conn.prep
- 删除 conn.exec_drop

### 插入
单条插入
```rust
    // 单条插入
    conn.exec_drop(
        r"INSERT INTO payment (customer_id, amount, account_name)
VALUES (:customer_id, :amount, :account_name)",
            params! {
                "customer_id" => 2,
                        "amount" => 99,
                        "account_name" => "clay",
            }
    )?;
```
批量插入
，首先定义插入数据的结构体
```rust
#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}
```
通过数组存储结构体数据，执行批量插入
```rust
    let payments: Vec<Payment> = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];
        conn.exec_batch(
            r"INSERT INTO payment (customer_id, amount, account_name)
    VALUES (:customer_id, :amount, :account_name)",
            payments.iter().map(|p| {
                params! {
                    "customer_id" => p.customer_id,
                            "amount" => p.amount,
                            "account_name" => &p.account_name,
                }
            }),
        )?;
```
预编译语句插入大量数据， conn.prep() + conn.exec_drop() 

不会判重，重复执行的话会插入多条同样的数据
```rust
    let insertstmt = conn.prep(
        r"INSERT INTO payment (customer_id, amount, account_name)
    VALUES (:customer_id, :amount, :account_name)",
    )?;
    for i in 10..15 {
        conn.exec_drop(
            &insertstmt,
            params! {
                "customer_id" => i,
                        "amount" => 99,
                        "account_name" => "clay",
            },
        )?;
    }
```
- 获取主键 conn.last_insert_id()
```rust
println!("新插入的记录的主键为: {}", conn.last_insert_id())
```
更新 conn.prep()
[Update](./examples/query_first.rs)

删除 conn.exec_drop()
```rust
{{#include examples/delete.rs}}
```
## 读操作
查询：query_iter + query_map
```rust
{{#include examples/query.rs}}
```
获取单条数据： query_first
查询特定数据行，可能会出现下面几种情况:

- 找到，返回实际数据
- 没有找到行
- 发生错误
所以，使用query_first函数返回的是Option的结果。 需要将其解包两次才可以获取实际的行数据:

条件查询： exec_first

```rust
{{#include examples/query_first.rs}}
```