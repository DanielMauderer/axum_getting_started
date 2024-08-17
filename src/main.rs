// rust axum main function

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("failed to bind to port 3000: {}", e);
            return;
        }
    };

    let url = match listener.local_addr() {
        Ok(addr) => format!("http://{}", addr),
        Err(e) => {
            eprintln!("failed to get the local address: {}", e);
            return;
        }
    };

    println!("listening on {}", url);
    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(e) => eprintln!("server error: {}", e),
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
