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
       // 更新数据
    let updatestmt = conn.prep(
        r"update payment set amount=:amount where customer_id=:customer_id and account_name=:account_name",
    )?;

        conn.exec_drop(
            &updatestmt,
            params! {
                "customer_id" => 2,
                        "amount" => 881,
                        "account_name" => "yalc",
            },
        )?;

        // let updatestmt = conn.prep(
        //     r"update payment set amount=:amount,account_name=:account_name where customer_id=:customer_id",
        // )?;
    
        //     conn.exec_drop(
        //         &updatestmt,
        //         params! {
        //             "customer_id" => 2,
        //                     "amount" => 881,
        //                     "account_name" => "yalc",
        //         },
        //     )?;
}