use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use uuid::Uuid;
use tera::{Tera, Context};

#[get("/")]
async fn index(template: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("game_id", &Uuid::new_v4());
    let rendered = template.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/game/{game_id}")]
async fn game(game_id: web::Path<Uuid>, template: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("game_id", &game_id.to_string());
    let rendered = template.render("game.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

/// Handler for static files
#[get("/static/{filename}")]
async fn staticfiles(request: HttpRequest) -> impl Responder {
    let filename = request.match_info().query("filename");
    let path: PathBuf = format!("./static/{}", filename).parse().unwrap();
    let file = NamedFile::open(path).unwrap();
    file.into_response(&request)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(tera.clone()))
                .service(index)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_game_get() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(tera.clone()))
                .service(game)
        ).await;
        let uri = format!("/game/{}", Uuid::new_v4());
        let req = test::TestRequest::get().uri(&uri).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_staticfiles_get() {
        let app = test::init_service(App::new().service(staticfiles)).await;
        let req = test::TestRequest::get().uri("/static/index.css").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}