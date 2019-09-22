use actix_web::{web, error, HttpResponse, Result};
use tera::{Context, Tera};

pub fn index(
    tmpl: web::Data<Tera>,
    ) -> Result<HttpResponse>{

    let mut context = Context::new();
    context.insert("greeting", "hello");

    let rendered = tmpl.render("index.html.tera", &context).map_err(|e| {
        error::ErrorInternalServerError(e.description().to_owned())
    })?;

    Ok(HttpResponse::Ok().body(rendered))
}
