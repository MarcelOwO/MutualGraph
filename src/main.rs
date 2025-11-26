mod api;
mod mutual;

#[tokio::main]
async fn main() {
    let config = api::Init(String::from("MutualGraph")).await;

    let player_id = "";

    let mut mutuals = mutual::get_mutuals(&config, player_id).await.unwrap();

    for mutual in mutuals.iter_mut() {
        println!("{}", mutual);
    }
}
