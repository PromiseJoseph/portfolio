use http_mambo::types::{HttpRequest, HttpResponse, StatusCode};
use std::{collections::HashMap, format, println};
/**
 *  A test handler for the home route.
 *  Handle an HTTP request and return an HTTP response.
 */
const DIR: &str = env!("CARGO_MANIFEST_DIR");

fn create_html_body() -> String {
    let base = include_str!("../templates/base.html");
    let header = include_str!("../templates/components/header.html");
    let introduction = include_str!("../templates/components/introduction.html");
    let projects = include_str!("../templates/components/project.html");
    let footer = include_str!("../templates/components/footer.html");
    let end = include_str!("../templates/end.html");
    let body = format!(
        r#"
        {base}       
            <div class="max-w-[1100px] mx-auto relative z-10 text-white">
            {header}
            <main class=" mx-auto px-8 pb-12">
                {introduction}{projects}
            </main>
            {footer}
            </div>
            {end}
    "#,
    );
    body.to_string()
}

pub async fn homepage(_request: HttpRequest) -> HttpResponse {
    let body = create_html_body();
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    // headers.insert("Server".to_string(), "HTTP Mambo".to_string());
    HttpResponse::new().with_body(body).with_headers(headers)
}

pub async fn styles(request: HttpRequest) -> HttpResponse {
    let paths = request.request_lines.path;
    let css_path = match paths.as_str() {
        "/index.css" => format!("{DIR}/static/css/index.css"),
        "/app.css" => format!("{DIR}/static/css/app.css"),
        _ => {
            return HttpResponse::with_status(StatusCode::OK)
                .with_body("CSS not found".to_string());
        }
    };
    let css = match std::fs::read_to_string(css_path) {
        Ok(css) => css,
        Err(_) => return HttpResponse::with_status(StatusCode::NOT_FOUND),
    };
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/css".to_string());

    HttpResponse::new()
        .with_headers(headers)
        .with_body(css.to_string())
}

pub async fn config(_request: HttpRequest) -> HttpResponse {
    let config = include_str!("../static/json/particlejs-config.json");
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    HttpResponse::new()
        .with_headers(headers)
        .with_body(config.to_string())
}

pub async fn scripts(request: HttpRequest) -> HttpResponse {
    let paths = request.request_lines.path;
    let script_path = match paths.as_str() {
        "/app.js" => format!("{DIR}/static/js/app.js",),
        _ => {
            return HttpResponse::with_status(StatusCode::OK)
                .with_body("Script not found".to_string());
        }
    };
    let script = match std::fs::read_to_string(script_path) {
        Ok(script) => script,
        Err(_) => return HttpResponse::with_status(StatusCode::NOT_FOUND),
    };

    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "application/javascript".to_string(),
    );

    HttpResponse::new()
        .with_headers(headers)
        .with_body(script.to_string())
}

pub async fn images(request: HttpRequest) -> HttpResponse {
    let path = request.request_lines.path;

    let image_path = match path.as_str() {
        "images/Holoshop.png" => format!("{DIR}/static/img/Holoshop.png"),
        "images/portfolio.png" => format!("{DIR}/static/img/portfolio.png"),
        "images/portfolio2.png" => format!("{DIR}/static/img/portfolio2.png"),
        "images/portfolio3.png" => format!("{DIR}/static/img/portfolio3.png"),
        _ => {
            return HttpResponse::with_status(StatusCode::NOT_FOUND)
                .with_body("Image not found".to_string());
        }
    };

    let image_data = std::fs::read(image_path).unwrap_or_else(|_| Vec::new());
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "image/png".to_string());

    HttpResponse::new()
        .with_headers(headers)
        .with_body(image_data)
}
