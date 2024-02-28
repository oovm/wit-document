use axum::{response::Html, routing::get, Router};
use dioxus::{dioxus_core::Mutations, prelude::*};
use std::{path::Path};
use wit_document::{renderer::render_interface, DataProvider};
use wit_parser::UnresolvedPackage;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    // build our application with a single route
    let app = Router::new().route("/", get(app_endpoint)).into_make_service();

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
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let store = DataProvider { package: UnresolvedPackage::parse_dir(&here.join("tests/preview2/cli")).unwrap() };
    let example = store.get_interfaces().into_iter().map(|x| render_interface(&store, x));
    rsx! {
        {example}
    }
}
