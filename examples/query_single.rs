use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mysql_url: &str = "mysql://root:123456@localhost:3306/testUser";
    let pool = Pool::new(mysql_url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"CREATE TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text)",
    )?;

    // 查询第一行数据中的单个数据字段
    let val: Option<String> = conn.query_first("SELECT account_name from payment")?;
    let account_name: String = val.clone().unwrap_or_default();
    println!("account_name = {account_name}");

    Ok(())
}
