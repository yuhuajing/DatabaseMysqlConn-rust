use mysql::prelude::*;
use mysql::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[derive(Clone)]
pub struct MyPool {
    //inner: Arc<inner::Inner>,
    pool: Arc<Pool>,
}

impl MyPool {
    pub fn new(mysql_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Ok(Self {
        //     inner: Arc::new(inner::Inner::new(Pool::new(mysql_url)?)),
        // })
        Ok(Self { pool:Arc::new(Pool::new(mysql_url)?) })
    }

    pub fn query_mysql(&self) -> std::result::Result<u64, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let teststr: String = String::from("0xff2B4721F997c242fF406a626f17df083Bd2C568");
        let mut query_state: &str = "SELECT blocknumber from transfer where address='{address}' order by blocknumber desc limit 1";
        let binding = query_state.replace("{address}", teststr.as_str());
        query_state = binding.as_str();
        let mut results = conn.query_iter(query_state)?;

        if let Some(row) = results.next() {
            let blocknumber: u64 = row?.get(0).unwrap_or(0);
            return Ok(blocknumber + 1);
        }
        Ok(0)
    }


    pub fn query_payment(&self) -> std::result::Result<u64, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let teststr: String = String::from("bar");
        let nu: u64 = 9;
        let mut query_state: &str = "SELECT count(*) from payment where customer_id={customer_id} and account_name='{account_name}'";
        let binding = query_state
            .replace("{customer_id}", nu.to_string().as_str())
            .replace("{account_name}", teststr.as_str());
        query_state = binding.as_str();    
        let mut results = conn.query_iter(query_state)?;
        if let Some(row) = results.next() {
            let blocknumber: u64 = row?.get(0).unwrap_or(0);
            return Ok(blocknumber);
        }
        Ok(0)
    }
}

// mod inner {
//     use mysql::Pool;

//     pub struct Inner {
//         pub pool: Pool,
//     }

//     impl Inner {
//         pub fn new(pool: Pool) -> Self {
//             Self { pool }
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mysql_url = "mysql://root:123456@localhost:3306/testUser";
    let mypool = MyPool::new(mysql_url)?;
    let mut count: u64 = 0;
    match mypool.query_mysql() {
        Ok(bn) => {
            count = bn;
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
    println!("Count: {}", count);

    match mypool.query_payment() {
        Ok(bn) => {
            count = bn;
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
    println!("Count: {}", count);
    // let _ = mypool.query_payment();
    Ok(())
}
