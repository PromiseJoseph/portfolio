mod handler;
use handler::{config, homepage, images, scripts, styles};

use http_mambo::types::{Client, Router};
use http_mambo::{listener, stream};
use std::sync::Arc;

const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let arg = std::env::args().nth(1);

    let custom_addr: &str = match arg.as_deref() {
        Some(addr) => addr,
        None => DEFAULT_SERVER_ADDRESS,
    };

    let listener = listener::bind_addresses(custom_addr).await.unwrap();

    println!("Server listening on: {:?}", listener.local_addr().unwrap());

    // Create a new router and register routes
    let mut router = Router::new();

    router.get("/", homepage);
    router.get("/index.css", styles);
    router.get("/app.css", styles);
    router.get("/particlejs-config.json", config);
    router.get("/app.js", scripts);
    router.get("/images/Holoshop.png", images);
    router.get("/images/Linkify.png", images);
    router.get("/images/Swiftze.png", images);
    router.get("/images/Wigo.png", images);
    router.get("/images/EchoMambo.png", images);
    router.get("/images/HttpMambo.png", images);
    router.get("/images/ChatMambo.png", images);
    router.get("/images/Portfolio.png", images);

    let router = Arc::new(router);

    loop {
        let (stream, peer_addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let (reader, writer) = stream.into_split();
        let client = Client { peer_addr };

        let router = Arc::clone(&router);

        tokio::spawn(async move {
            if let Err(e) = stream::handle_stream(reader, writer, client, router).await {
                eprintln!("Error handling stream: {}", e);
            }
        });
    }
}
