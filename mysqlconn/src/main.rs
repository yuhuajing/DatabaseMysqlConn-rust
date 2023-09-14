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
    // let transfer_query = format!(query_state, 9);
    let teststr: String = String::from("bar");
    let nu: u64 = 9;
    let mut query_state: &str = "SELECT count(*) from payment where customer_id={customer_id} and account_name='{account_name}'";
    let binding = query_state
        .replace("{customer_id}", nu.to_string().as_str())
        .replace("{account_name}", teststr.as_str());
    query_state = binding.as_str();

    // println!("{}", query_state);

    // let val: Vec<Payment> = conn.query_map(
    //     //"SELECT customer_id, amount, account_name from payment where account_name='clay'",
    //     query_state,
    //     |(customer_id, amount, account_name)| Payment {
    //         customer_id,
    //         amount,
    //         account_name,
    //     },
    // )?;

    // if val.iter().len() > 0 {
    //     println!("{}", val.iter().len());
    // }

    let teststr: String = String::from("0xff2B4721F997c242fF406a626f17df083Bd2C568");
    let mut query_state: &str = "SELECT blocknumber from transfer where address='{address}' order by blocknumber desc limit 1";
    let binding = query_state.replace("{address}", teststr.as_str());
    query_state = binding.as_str();
    // conn.query_iter(query_state)
    // .unwrap()
    // .for_each(|row| {
    //     let r:i64 = from_row(row.unwrap());
    //     println!("{}", r);
    // });

    let results = conn.query_iter(query_state);

    if let Some(row) = results?.next() {
        let count: u64 = row?.get(0).unwrap_or(0);
        println!("Count: {}", count);
    } else {
        println!("No results found.");
    }

    Ok(())
}
