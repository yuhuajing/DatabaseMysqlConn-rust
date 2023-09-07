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

// 不能有空数据，否则会解析失败
    conn.query_iter("SELECT customer_id, amount, account_name from payment")
        .unwrap()
        .for_each(|row| {
            let r: (i32, i32, String ) = from_row(row.unwrap());
            println!("{}, {},{}", r.0, r.1, r.2);
        });

    //查询整行数据，返回的数据和表结构一致
    let val: Vec<Payment> = conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        },
    )?;

    println!("{} data found!", val.iter().len());
    for log in val.iter() {
        let customer_id = log.customer_id;
        let amount = log.amount;
        let account_name: String = log.account_name.clone().unwrap_or_default(); // 可以有空数据
        println!("customer_id={customer_id}, amount = {amount}, account_name = {account_name}");
    }

    Ok(())
}