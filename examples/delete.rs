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
    // 删除数据
    let deletestmt =
        conn.prep(r"delete from payment where customer_id=:customer_id and account_name=:account_name")?;

    conn.exec_drop(
        &deletestmt,
        params! {
            "customer_id" => 2,
                    "account_name" => "yalc",
        },
    )?;

    Ok(())
}
