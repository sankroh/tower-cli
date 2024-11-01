use clap::Command;
use config::Config;
use tower_api::Client;
use promptly::prompt;
use crate::output;

pub fn login_cmd() -> Command {
    Command::new("login")
        .about("Create a session with Tower")
}

pub async fn do_login(_config: Config, client: Client) {
    let email: String = prompt("Email").unwrap();
    let password: String = prompt("Password").unwrap();

    match client.login(&email, &password).await {
        Ok(session) => {
            if let Err(err) = session.save() {
                output::config_error(err);
            } else {
                output::success(format!("Hello, {}!", session.user.email).as_str());
            }
        },
        Err(err) => {
            output::tower_error(err);
        }
    }
}
