use actix_web::{web, error, HttpResponse, Result};
use tera::{Context, Tera};

pub async fn index(
    tmpl: web::Data<Tera>,
    ) -> Result<HttpResponse>{
    
    let mut context = Context::new();

    context.insert("TITLE", &dotenv!("TITLE"));
    context.insert("VERSION", &dotenv!("VERSION"));
    let rendered = tmpl.render("index.html.tera", &context).map_err(|e| {
        error::ErrorInternalServerError(e.to_string())
    })?;

    Ok(HttpResponse::Ok().body(rendered))
}
