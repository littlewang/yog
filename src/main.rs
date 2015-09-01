#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

extern crate markdown;

extern crate mysql;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


use std::default::Default;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;



struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}


//mysql db
fn conn_db(){

        println!("PORT IS {}", "3306");

        let opts = MyOpts {
              user: Some("root".to_string()),
              pass: Some("sakai".to_string()),
              ..Default::default()
        };

        let pool = MyPool::new(opts).unwrap();
        // Let's create payment table.
        // It is temporary so we do not need `tmp` database to exist.
        // Unwap just to make sure no error happened.
        pool.prep_exec(r"CREATE TEMPORARY TABLE tmp.payment (
                             customer_id int not null,
                             amount int not null,
                             account_name text
                         )", ()).unwrap();

        let payments = vec![
            Payment { customer_id: 1, amount: 2, account_name: None },
            Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
            Payment { customer_id: 5, amount: 6, account_name: None },
            Payment { customer_id: 7, amount: 8, account_name: None },
            Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
        ];

        // Let's insert payments to the database
        // We will use into_iter() because we do not need to map Stmt to anything else.
        // Also we assume that no error happened in `prepare`.
        for mut stmt in pool.prepare(r"INSERT INTO tmp.payment
                                           (customer_id, amount, account_name)
                                       VALUES
                                           (?, ?, ?)").into_iter() {
            for p in payments.iter() {
                // `execute` takes ownership of `params` so we pass account name by reference.
                // Unwrap each result just to make sure no errors happended.
                stmt.execute((p.customer_id, p.amount, &p.account_name)).unwrap();
            }
        }

        // Let's select payments from database
        let selected_payments: Vec<Payment> =
        pool.prep_exec("SELECT customer_id, amount, account_name from tmp.payment", ())
        .map(|result| { // In this closure we sill map `QueryResult` to `Vec<Payment>`
            // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
            // will map each `MyResult` to contained `row` (no proper error handling)
            // and second call to `map` will map each `row` to `Payment`
            result.map(|x| x.unwrap()).map(|row| {
                let (customer_id, amount, account_name) = from_row(row);
                Payment {
                    customer_id: customer_id,
                    amount: amount,
                    account_name: account_name,
                }
            }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        }).unwrap(); // Unwrap `Vec<Payment>`

        // Now make shure that `payments` equals to `selected_payments`.
        // Mysql gives no guaranties on order of returned rows without `ORDER BY`
        // so assume we are lukky.
        // assert_eq!(payments, selected_payments);
        println!("Vector size: {}" , selected_payments.len());
        println!("Yay!");
}



fn main() {


    conn_db();

    let mut server = Nickel::new();

    //outprint html by markdown
    let html : String = markdown::to_html("__I am markdown__");

    // input and println
    let path = Path::new("pages/index.html");

    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {

    // The `description` method of `io::Error` returns a string that
    // describes the error
    Err(why) => panic!("couldn't open {}: {}", display,
                                               Error::description(&why)),
    Ok(file) => file,
        };

    let mut s = String::new();

    match file.read_to_string(&mut s) {

       Err(why) => panic!("couldn't read {}: {}", display,
                                      Error::description(&why)),

       Ok(_) => print!("{} contains:\n{}", display, s),

   }






    server.get("/",middleware!(&*s));

    server.listen("127.0.0.1:6767");

}
