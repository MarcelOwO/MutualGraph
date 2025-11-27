use std::io::{self, Write};
use std::sync::mpsc::Sender;
use std::{thread::sleep, time::Duration};

use vrchatapi::{
    apis,
    apis::configuration,
    models::{EitherUserOrTwoFactor, LimitedUserFriend, TwoFactorAuthCode, TwoFactorEmailCode},
};

use crate::api::custom::get_mutuals;
use crate::db::models::Friend;
use crate::utility::read_user_input;

pub async fn authenticate(application: String) -> configuration::Configuration {
    let email = read_user_input("enter email");
    let username = read_user_input("enter username");
    let password = read_user_input("enter password");

    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((username, Some(password)));

    let user_string = format!("{0} {1}", application, email);

    config.user_agent = Some(user_string);

    match apis::authentication_api::get_current_user(&config)
        .await
        .unwrap()
    {
        vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(me) => {
            println!("Username: {}", me.username.unwrap())
        }
        vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            if requires_auth
                .requires_two_factor_auth
                .contains(&String::from("emailOtp"))
            {
                let code = email;
                if let Err(err) = apis::authentication_api::verify2_fa_email_code(
                    &config,
                    TwoFactorEmailCode::new(code),
                )
                .await
                {
                    eprintln!("Error verifying 2FA email code: {}", err);
                }
            } else {
                let code = read_user_input("Please enter your Authenticator 2fa code: ");
                if let Err(err) =
                    apis::authentication_api::verify2_fa(&config, TwoFactorAuthCode::new(code))
                        .await
                {
                    eprintln!("Error verifying 2FA auth code: {}", err);
                }
            }
        }
    }

    let user = apis::authentication_api::get_current_user(&config)
        .await
        .unwrap();

    match user {
        EitherUserOrTwoFactor::CurrentUser(user) => {
            println!("Current user: {}", user.display_name);
            for friend in user.friends {
                println!("{}", friend)
            }
        }
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => println!("cookie invalid"),
    }

    config
}

pub(crate) async fn init(tx: Sender<Friend>) {
    let config = authenticate(String::from("MutualGraph")).await;

    let total_friends = 300;

    let mut offset = 0;
    let n = 100;

    let mut friends: Vec<LimitedUserFriend> = vec![];

    while offset <= total_friends {
        match vrchatapi::apis::friends_api::get_friends(&config, Some(offset), Some(n), Some(true))
            .await
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

    for friend in friends {
        let mut mutuals: Vec<LimitedUserFriend> = vec![];
        let mut newFriend = Friend::default();
        let player_uuid = friend.id;
        newFriend.uuid = player_uuid.clone();
        match get_mutuals(&config, player_uuid).await {
            Ok(result) => {
                for mutual in result {
                    mutuals.push(mutual);
                }
            }
            Err(e) => {
                println!("Error getting mutual: {}", e);
            }
        };

        let uuid_list: Vec<String> = mutuals.iter().map(|x| x.id.clone()).collect();
        newFriend.friends.extend(uuid_list);

        match tx.send(newFriend) {
            Ok(result) => {
                // nothing
            }
            Err(e) => {
                println!("{}", e)
            }
        }
        //Delay because of vrchat api rules
        sleep(Duration::from_secs(60));
    }
}
