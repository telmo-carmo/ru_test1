/*

RUST several tests

*/

use std::time::SystemTime;

fn main() {
    std::env::set_var("RUST_LOG", "ru_test1"); // log ALL
    env_logger::init();
    println!("Hello,{}", "TEST");
    let now = SystemTime::now();
    log::warn!("This is now {now:?}");
    log::debug!("THE END!");
}
