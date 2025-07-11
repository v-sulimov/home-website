use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_files;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(
            App::new().service(actix_files::Files::new("/", "./static").index_file("index.html")),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = str::from_utf8(body.as_ref()).expect("Response body is not valid UTF-8");
        assert!(body_str.contains("Vitaly Sulimov home on the internet."));
        assert!(body_str.contains(r#"<a href="https://git.vsulimov.com">git</a>"#));
        assert!(body_str.contains(r#"<a href="https://maven.vsulimov.com">maven</a>"#));
        assert!(
            body_str.contains(r#"<a href="https://mirror.vsulimov.com">mirror.vsulimov.com</a>"#)
        );
        assert!(body_str.contains(r#"<a href="https://instagram.com/open.climb">Instagram</a>"#));
    }
}
