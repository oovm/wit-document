use std::thread::Scope;
use axum::{response::Html, routing::get, Router};
use dioxus::dioxus_core::Mutations;
use dioxus::prelude::*;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(app_endpoint))
        .into_make_service();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn app_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(app);
    let mut mutations = Mutations::default();
    app.rebuild(&mut mutations);
    Html(dioxus_ssr::render(&app))
}

// create a component that renders a div with the text "hello world"
fn app() -> Element {
    rsx!( div { "hello world" } )
}