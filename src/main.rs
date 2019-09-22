#[macro_use]
extern crate tera;

use actix_files as fs;
use tera::Tera;
use actix_web::{middleware, web, App, HttpServer};
use listenfd::ListenFd;

mod api;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        let templates: Tera = compile_templates!("templates/**/*");
        App::new()
            // enable logger
            .data(templates)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(api::index))
            .service(fs::Files::new("/static", "static/"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    }else{
        server.bind("0.0.0.0:8080").unwrap()
    };

    server.run().unwrap()
} 
