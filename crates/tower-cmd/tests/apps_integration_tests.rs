use tower_api::Client;
use config::{Config, Session, Token, User};
use tokio::test;
use std::time::Duration;
use tower_cmd::apps::*;
use url::Url;
use dotenv::dotenv;
use std::env;
use chrono::Utc;

// Helper to ensure we have a logged-in client
fn get_test_client() -> Client {
    // Load from .env file if present
    dotenv().ok();

    let email = env::var("TOWER_TEST_EMAIL")
        .expect("TOWER_TEST_EMAIL must be set");
    let token = env::var("TOWER_TEST_TOKEN")
        .expect("TOWER_TEST_TOKEN must be set");
    let tower_url = env::var("TOWER_URL")
        .expect("TOWER_URL must be set");

    let url = Url::parse(&tower_url).expect("Invalid TOWER_URL");

    let session = Session {
        tower_url: url,
        user: User { 
            email,
            created_at: Utc::now().to_rfc3339()
        },
        token: Token { jwt: token },
    };

    Client::default().with_optional_session(Some(session))
}

#[test]
async fn test_app_lifecycle() {
    let client = get_test_client();
    let test_app_name = "test-app-integration";
    let test_description = "Integration test app";
    let test_schedule = "0 * * * *";
    
    // First create an app
    let create_cmd = apps_cmd();
    let create_matches = create_cmd.get_matches_from(vec![
        "apps",
        "create",
        "--name", test_app_name,
        "--description", test_description,
        "--schedule", test_schedule
    ]);

    if let Some(("create", args)) = create_matches.subcommand() {
        do_create_app(Config::default(), client.clone(), args).await;
    }

    // Give the API a moment to process
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Verify the app was created with correct parameters
    match client.get_app(test_app_name).await {
        Ok((app, _runs)) => {
            assert_eq!(app.name, test_app_name, "App name doesn't match");
            assert_eq!(app.short_description, test_description, "App description doesn't match");
            assert_eq!(app.schedule, test_schedule, "App schedule doesn't match");
        },
        Err(e) => panic!("Failed to get app details: {}", e),
    }

    // Verify it shows up in the list
    let list_cmd = apps_cmd();
    let list_matches = list_cmd.get_matches_from(vec![
        "apps",
        "list"
    ]);

    if let Some(("list", _)) = list_matches.subcommand() {
        // Instead of calling do_list_apps directly, we'll use the client to get the list
        match client.list_apps().await {
            Ok(apps) => {
                let found_app = apps.iter().find(|app| app.app.name == test_app_name);
                assert!(found_app.is_some(), "Created app not found in list");
                
                let app = found_app.unwrap();
                assert_eq!(app.app.short_description, test_description, "App description doesn't match in list");
                assert_eq!(app.app.schedule, test_schedule, "App schedule doesn't match in list");
            },
            Err(e) => panic!("Failed to list apps: {}", e),
        }
    }

    // Show the app details
    let show_cmd = apps_cmd();
    let show_matches = show_cmd.get_matches_from(vec![
        "apps",
        "show",
        test_app_name
    ]);

    if let Some(("show", args)) = show_matches.subcommand() {
        do_show_app(Config::default(), client.clone(), Some((test_app_name, args))).await;
    }

    // Clean up - delete the test app
    let delete_cmd = apps_cmd();
    let delete_matches = delete_cmd.get_matches_from(vec![
        "apps",
        "delete",
        test_app_name
    ]);

    if let Some(("delete", args)) = delete_matches.subcommand() {
        do_delete_app(Config::default(), client.clone(), Some((test_app_name, args))).await;
    }
}

#[test]
async fn test_invalid_app_operations() {
    let client = get_test_client();
    let nonexistent_app = "this-app-does-not-exist";

    // Try to show a nonexistent app
    let show_cmd = apps_cmd();
    let show_matches = show_cmd.get_matches_from(vec![
        "apps",
        "show",
        nonexistent_app
    ]);

    if let Some(("show", args)) = show_matches.subcommand() {
        do_show_app(Config::default(), client.clone(), Some((nonexistent_app, args))).await;
    }

    // Try to get logs from a nonexistent app
    let logs_cmd = apps_cmd();
    let logs_matches = logs_cmd.get_matches_from(vec![
        "apps",
        "logs",
        &format!("{}#1", nonexistent_app)
    ]);

    if let Some(("logs", args)) = logs_matches.subcommand() {
        do_logs_app(Config::default(), client.clone(), Some((&format!("{}#1", nonexistent_app), args))).await;
    }

    // Try to delete a nonexistent app
    let delete_cmd = apps_cmd();
    let delete_matches = delete_cmd.get_matches_from(vec![
        "apps",
        "delete",
        nonexistent_app
    ]);

    if let Some(("delete", args)) = delete_matches.subcommand() {
        do_delete_app(Config::default(), client.clone(), Some((nonexistent_app, args))).await;
    }
}
