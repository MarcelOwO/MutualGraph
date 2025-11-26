use std::io::{self, Write};
pub use vrchatapi::apis;
use vrchatapi::{
    apis::{configuration, friends_api, users_api},
    models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode},
};

pub async fn Init(application: String) -> configuration::Configuration {
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

fn read_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
