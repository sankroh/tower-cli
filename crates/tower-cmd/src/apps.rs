use colored::Colorize;
use clap::{value_parser, Arg, ArgMatches, Command};
use config::Config;
use tower_api::Client;

use crate::output;

pub fn apps_cmd() -> Command {
    Command::new("apps")
        .about("Interact with the apps that you own")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("List all of your apps`")
        )
        .subcommand(
            Command::new("show")
                .allow_external_subcommands(true)
                .about("Show the details about an app in Tower")
        )
        .subcommand(
            Command::new("logs")
                .allow_external_subcommands(true)
                .about("Get the logs from a previous Tower app run")
        )
        .subcommand(
            Command::new("create")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .value_parser(value_parser!(String))
                        .action(clap::ArgAction::Set)
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .value_parser(value_parser!(String))
                        .default_value("")
                        .action(clap::ArgAction::Set)
                )
                .about("Create a new app in Tower")
        )
        .subcommand(
            Command::new("delete")
                .allow_external_subcommands(true)
                .about("Delete an app in Tower")
        )
}

pub async fn do_logs_app(_config: Config, client: Client, cmd: Option<(&str, &ArgMatches)>) {
    let opts = cmd.unwrap_or_else(|| {
        output::die("App name (e.g. tower apps show <name>#<num>) is required");
    });

    let (app_name, num) = if opts.0.contains("#") {
        let parts: Vec<&str> = opts.0.split("#").collect();
        (parts[0], parts[1])
    } else {
        output::die("Run number is required (e.g. tower apps show <name>#<num>)");
    };

    let spinner = output::spinner("Fetching logs...");

    match client.get_run_logs(&app_name, num).await {
        Ok(logs) => {
            spinner.success();

            for li in logs.iter() {
                output::log_line(&li.timestamp, &li.message, output::LogLineType::Remote);
            }
        }, Err(err) => {
            spinner.failure();

            output::tower_error(err);
        }
    }
}

pub async fn do_show_app(_config: Config, client: Client, cmd: Option<(&str, &ArgMatches)>) {
    let opts = cmd.unwrap_or_else(|| {
        output::die("App name (e.g. tower apps show <name>) is required");
    });

    match client.get_app(&opts.0).await {
        Ok((app, runs)) => {
            let line = format!("{} {}\n", "Name:".bold().green(), app.name);
            output::write(&line);

            let line = format!("{}\n", "Description:".bold().green());
            output::write(&line);

            let line = output::paragraph(&app.short_description);
            output::write(&line);

            output::newline();
            output::newline();

            let line = format!("{}\n", "Recent runs:".bold().green());
            output::write(&line);
    
            let headers = vec![
                "#".yellow().to_string(),
                "Status".yellow().to_string(),
                "Start Time".yellow().to_string(),
                "Elapsed Time".yellow().to_string(),
            ];

            let rows = runs.iter().map(|run| {
                let status = run.status.clone();
                let start_time = run.started_at.unwrap().to_string();
                let elapsed_time = "".to_string();

                vec![
                    run.number.to_string(),
                    status.to_string(),
                    start_time,
                    elapsed_time,
                ]
            }).collect();

            output::table(headers, rows);
        },
        Err(err) => {
            output::tower_error(err);
        }
    }
}

pub async fn do_list_apps(_config: Config, client: Client) {
    let res = client.list_apps().await;

    match res {
        Ok(apps) => {
            let items = apps.iter().map(|sum| {
                let desc = sum.app.short_description.clone();
                let desc = if desc.is_empty() {
                    "No description".white().dimmed().italic()
                } else { 
                    desc.normal().clear()
                };

                format!("{}\n{}", sum.app.name.bold().green(), desc)
            }).collect();

            output::list(items);
        },
        Err(err) => {
            output::tower_error(err);
        }
    }
}

pub async fn do_create_app(_config: Config, client: Client, args: &ArgMatches) {
    let name = args.get_one::<String>("name").unwrap_or_else(|| {
        output::die("App name (--name) is required");
    });

    let description = args.get_one::<String>("description").unwrap();

    let spinner = output::spinner("Creating app");

    match client.create_app(&name, &description).await {
        Ok(_app) => {
            spinner.success();

            let line = format!("App \"{}\" was created", name);
            output::success(&line);
        },
        Err(err) => {
            spinner.failure();

            output::tower_error(err);
        }
    }
}

pub async fn do_delete_app(_config: Config, client: Client, cmd: Option<(&str, &ArgMatches)>) {
    let opts = cmd.unwrap_or_else(|| {
        output::die("App name (e.g. tower apps delete <name>) is required");
    });

    let spinner = output::spinner("Deleting app...");

    match client.delete_app(&opts.0).await {
        Ok(_app) => {
            spinner.success();

            let line = format!("App \"{}\" was deleted", &opts.0);
            output::success(&line);
        },
        Err(err) => {
            spinner.failure();

            output::tower_error(err);
        }
    }
}
