use std::{thread::sleep, time::Duration};

use api::init;
use vrchatapi::models::LimitedUserFriend;
mod api;
mod mutual;

#[tokio::main]
async fn main() {
    let config = init(String::from("MutualGraph")).await;

    let total_friends = 300;

    let mut offset = 0;
    let n = 100;

    let mut friends: Vec<LimitedUserFriend> = vec![];

    while offset <= total_friends {
        match api::apis::friends_api::get_friends(&config, Some(offset), Some(n), Some(true)).await
        {
            Ok(result) => {
                for friend in result {
                    friends.push(friend);
                }
            }
            Err(e) => {
                println!("Failed to get Friends: {}", e);
            }
        };

        offset += friends.len() as i32;
        //Delay because of vrchat api rules
        sleep(Duration::from_secs(60));
    }

    let mut mutuals: Vec<LimitedUserFriend> = vec![];

    for friend in friends {
        let player_uuid = friend.id;

        match mutual::get_mutuals(&config, player_uuid).await {
            Ok(result) => {
                for mutual in result {
                    mutuals.push(mutual);
                }
            }
            Err(e) => {
                println!("Error getting mutual: {}", e);
            }
        };

        //Delay because of vrchat api rules
        sleep(Duration::from_secs(60));
    }
}
