 // use chrono::prelude::*;
// use chrono::{Utc,Local};


// fn main() {
//     println!("Hello, world!");
//     let now = Utc::now();
//     println!("UTC: {}", now);
//     let local = Local::now();
//     println!("Local: {}", local);
// }


use dotenv::dotenv;
use std::env;


fn main(){
    dotenv().ok();
    let var = env::var("REDIS_URL");

    match var{
        Ok(v) => println!("The value of the var is: {}", v),
        Err(e) => println!("Couldn't read the var: {}", e)
    }
}
