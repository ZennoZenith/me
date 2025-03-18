use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use me::{configuration::CONFIGURATION, uitls::TEMPLATES};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};
use tera::Context;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let host = CONFIGURATION.application.host.as_str();
    let port = CONFIGURATION.application.port;
    let app = Router::new()
        .route("/", get(index))
        .nest("/assets", server_assets())
        .route_service("/favicon.ico", favicon());

    // write address like this to not make typos
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::from_str(host).expect("Invalid host")),
        port,
    );
    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to bind to host:port");

    println!("Listning from {}:{}", host, port);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Result<impl IntoResponse, (StatusCode, String)> {
    let context = Context::new();
    TEMPLATES
        .render("index.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

fn favicon() -> ServeFile {
    ServeFile::new("assets/images/favicon.ico")
}

fn server_assets() -> Router {
    const NOT_FOUND_HTML_PATH: &str = "assets/html/not_found.html";
    let serve_js_dir =
        ServeDir::new("assets/js").not_found_service(ServeFile::new(NOT_FOUND_HTML_PATH));
    let serve_css_dir =
        ServeDir::new("assets/css").not_found_service(ServeFile::new(NOT_FOUND_HTML_PATH));
    let serve_html_dir =
        ServeDir::new(NOT_FOUND_HTML_PATH).not_found_service(ServeFile::new(NOT_FOUND_HTML_PATH));
    let serve_image_dir =
        ServeDir::new("assets/image").not_found_service(ServeFile::new(NOT_FOUND_HTML_PATH));

    Router::new()
        .nest_service("/js", serve_js_dir)
        .nest_service("/css", serve_css_dir)
        .nest_service("/html", serve_html_dir)
        .nest_service("/image", serve_image_dir)
}
