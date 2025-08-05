use axum::{
    Router,
    http::{StatusCode, Uri},
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
        .route("/", get(home))
        .route("/projects", get(projects))
        .route("/showcases", get(showcases))
        .nest("/assets", server_assets())
        .route_service("/favicon.ico", favicon())
        .fallback(fallback_not_found);

    // write address like this to not make typos
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::from_str(host).expect("Invalid host")),
        port,
    );
    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to bind to host:port");

    println!("Listning from {host}:{port}");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn fallback_not_found(uri: Uri) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut context = Context::new();
    context.insert("uri", uri.to_string().as_str());

    let t = TEMPLATES
        .render("error404.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::NOT_FOUND, t))
}

async fn home() -> Result<impl IntoResponse, (StatusCode, String)> {
    let context = Context::new();
    TEMPLATES
        .render("home.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn projects() -> Result<impl IntoResponse, (StatusCode, String)> {
    let context = Context::new();
    TEMPLATES
        .render("projects.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn showcases() -> Result<impl IntoResponse, (StatusCode, String)> {
    let context = Context::new();
    TEMPLATES
        .render("showcases.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

fn favicon() -> ServeFile {
    ServeFile::new("assets/images/favicon.ico")
}

fn server_assets() -> Router {
    let serve_js_dir = ServeDir::new("assets/js");
    let serve_css_dir = ServeDir::new("assets/css");
    let serve_html_dir = ServeDir::new("assets/html");
    let serve_image_dir = ServeDir::new("assets/images");

    Router::new()
        .nest_service("/js", serve_js_dir)
        .nest_service("/css", serve_css_dir)
        .nest_service("/html", serve_html_dir)
        .nest_service("/images", serve_image_dir)
}
