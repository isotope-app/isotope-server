extern crate config;
use std::collections::HashMap;

pub fn get_settings()-> HashMap<String, String>{
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    return settings.try_into::<HashMap<String,String>>().unwrap();
}
