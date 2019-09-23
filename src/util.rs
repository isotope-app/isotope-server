extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

pub fn get_settings()->{
    dotenv().ok();

    return env::vars();
}
