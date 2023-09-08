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

    // 查询单条特定规则的数据
    let val: Option<String> = conn.exec_first(
        r"SELECT account_name from payment where customer_id=:customer_id",
        params! {
            "customer_id" => 2,
        },
    )?;
    let account_name: String = val.clone().unwrap_or_default();
    println!("account_name = {account_name}");

    // let val: Option<String> =
    //     conn.query_first("SELECT account_name from payment where customer_id=9")?;
    // let account_name:String = val.clone().unwrap_or_default();
    
    // if "bar" == account_name{
    //     println!("account_name = {account_name}");
    // }

    Ok(())
}
