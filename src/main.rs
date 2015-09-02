#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody,StaticFilesHandler};

extern crate markdown;

extern crate mysql;

extern crate rustc_serialize;


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

#[derive(RustcDecodable, RustcEncodable)]
struct Blog {
id:i32,
title: Option<String>,
content:Option<String>,
}



//mysql db
fn  conn_db()->MyPool{
        println!("PORT IS {}", "3306");
        let opts = MyOpts {
              user: Some("root".to_string()),
              pass: Some("sakai".to_string()),
              db_name: Some("yog".to_string()),
              ..Default::default()
        };
        let pool = MyPool::new(opts).unwrap();
        return pool;
}

// function with args

fn insertBlog(blog:Blog){
    let  pool=conn_db();

let mut stmt0 = pool.prepare(r"INSERT INTO blog (title,content) VALUES (?,?,?)").unwrap();

stmt0.execute((blog.title,blog.content)).unwrap();

}


fn main() {

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


    server.post("/addBlog",middleware!{|request,response|

        // let log = request.json_as::<Blog>().unwrap();
        // insert bolg
        // insertBlog(log);
        // format!("Hello {}", request.param("title").unwrap())
        println!("logging request: {:?}", request.param("title"));

        format!("You didn't provide any foo values!")

        });

    server.get("/",middleware!(&*s));


    // server.get("/back",);

    server.utilize(StaticFilesHandler::new("pages/assets/"));


    server.listen("127.0.0.1:6767");


}
