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
    // conn.query_drop(
    //     r"CREATE TABLE payment (
    //     customer_id int not null,
    //     amount int not null,
    //     account_name text)",
    // )?;

    //     // 单条插入
    //     conn.exec_drop(
    //         r"INSERT INTO payment (customer_id, amount, account_name)
    // VALUES (:customer_id, :amount, :account_name)",
    //             params! {
    //                 "customer_id" => 2,
    //                         "amount" => 99,
    //                         "account_name" => "yhj",
    //             }
    //     )?;

    // let payments: Vec<Payment> = vec![
    //     Payment {
    //         customer_id: 1,
    //         amount: 2,
    //         account_name: None,
    //     },
    //     Payment {
    //         customer_id: 3,
    //         amount: 4,
    //         account_name: Some("foo".into()),
    //     },
    //     Payment {
    //         customer_id: 5,
    //         amount: 6,
    //         account_name: None,
    //     },
    //     Payment {
    //         customer_id: 7,
    //         amount: 8,
    //         account_name: None,
    //     },
    //     Payment {
    //         customer_id: 9,
    //         amount: 10,
    //         account_name: Some("bar".into()),
    //     },
    // ];
    //     conn.exec_batch(
    //         r"INSERT INTO payment (customer_id, amount, account_name)
    // VALUES (:customer_id, :amount, :account_name)",
    //         payments.iter().map(|p| {
    //             params! {
    //                 "customer_id" => p.customer_id,
    //                         "amount" => p.amount,
    //                         "account_name" => &p.account_name,
    //             }
    //         }),
    //     )?;

    // 预编译插入的执行语句
    // let insertstmt = conn.prep(
    //     r"INSERT INTO payment (customer_id, amount, account_name)
    // VALUES (:customer_id, :amount, :account_name)",
    // )?;
    // for i in 10..15 {
    //     conn.exec_drop(
    //         &insertstmt,
    //         params! {
    //             "customer_id" => i,
    //                     "amount" => 99,
    //                     "account_name" => "clay",
    //         },
    //     )?;
    // }

    //    // 更新数据
    // let updatestmt = conn.prep(
    //     r"update payment set amount=:amount where customer_id=:customer_id and account_name=:account_name",
    // )?;

    //     conn.exec_drop(
    //         &updatestmt,
    //         params! {
    //             "customer_id" => 2,
    //                     "amount" => 881,
    //                     "account_name" => "yalc",
    //         },
    //     )?;
    // // 删除数据
    // let deletestmt =
    //     conn.prep(r"delete from payment where customer_id=:customer_id and account_name=:account_name")?;

    // conn.exec_drop(
    //     &deletestmt,
    //     params! {
    //         "customer_id" => 2,
    //                 "account_name" => "yalc",
    //     },
    // )?;


    conn.query_iter("SELECT customer_id, amount, account_name from payment")
        .unwrap()
        .for_each(|row| {
            let r: (i32, i32, String ) = from_row(row.unwrap());
            println!("{}, {},{}", r.0, r.1, r.2);
        });

    // //查询整行数据，返回的数据和表结构一致
    // let val: Vec<Payment> = conn.query_map(
    //     "SELECT customer_id, amount, account_name from payment",
    //     |(customer_id, amount, account_name)| Payment {
    //         customer_id,
    //         amount,
    //         account_name,
    //     },
    // )?;

    // println!("{} data found!", val.iter().len());
    // for log in val.iter() {
    //     let customer_id = log.customer_id;
    //     let amount = log.amount;
    //     let account_name: String = log.account_name.clone().unwrap_or_default();
    //     println!("customer_id={customer_id}, amount = {amount}, account_name = {account_name}");
    // }

    // // 查询第一行数据中的单个数据字段
    // let val: Option<String> = conn.query_first("SELECT account_name from payment")?;
    // let account_name: String = val.clone().unwrap_or_default();
    // println!("account_name = {account_name}");

    Ok(())
}
