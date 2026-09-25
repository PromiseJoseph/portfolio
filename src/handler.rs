use http_mambo::types::{HttpRequest, HttpResponse};
use std::collections::HashMap;
/**
 *  A test handler for the home route.
 *  Handle an HTTP request and return an HTTP response.
 */

fn create_html_body() -> String {
    let base = include_str!("../templates/base.html");
    let header = include_str!("../templates/components/header.html");
    let introduction = include_str!("../templates/components/introduction.html");
    let projects = include_str!("../templates/components/project.html");
    let footer = include_str!("../templates/components/footer.html");
    let body = format!(
        r#"
        {base}
        <body>
        <div id="particles-js"></div>
        <div id="root">
            <div className="max-w-[1100px] mx-auto relative z-10 text-white">
            {header}
            <main className=" mx-auto px-8 pb-12">
                {introduction}{projects}
            </main>
            {footer}
        </div>
        </div>
        <script src="https://cdn.jsdelivr.net/particles.js/2.0.0/particles.min.js"></script>
        </body>
        </html>
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

pub async fn styles(_request: HttpRequest) -> HttpResponse {
    let css = include_str!("../static/index.css");
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/css".to_string());

    HttpResponse::new()
        .with_headers(headers)
        .with_body(css.to_string())
}
