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
        // 单条插入
        conn.exec_drop(
            r"INSERT INTO payment (customer_id, amount, account_name)
    VALUES (:customer_id, :amount, :account_name)",
                params! {
                    "customer_id" => 2,
                            "amount" => 99,
                            "account_name" => "yhj",
                }
        )?;


    Ok(())
}
