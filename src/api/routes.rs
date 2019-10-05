use actix_web::{web};
use actix_files as fs;

use crate::api::{index::index};

pub fn routes(app: &mut web::ServiceConfig) {
	app
		.service(web::resource("/").to(index))
		.service(fs::Files::new("/static", "static/"));
}