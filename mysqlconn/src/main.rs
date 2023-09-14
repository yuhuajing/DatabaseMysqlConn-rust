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

   // 预编译插入的执行语句
   let query_state :&str = r"INSERT INTO payment (customer_id, amount, account_name) VALUES (:customer_id, :amount, :account_name)";
    let insertstmt = conn.prep(
        query_state,
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



    Ok(())
}