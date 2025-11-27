mod api;
mod utility;

#[tokio::main]
async fn main() {
    api::functions::init().await;
}
